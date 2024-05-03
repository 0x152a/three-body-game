use std::cmp::Ordering;
use macroquad::prelude::*;
use crate::body::BodyLike;
use crate::controller::Controller;
use crate::num::{abs, num, Num, max, min};
use std::collections::VecDeque;
use std::time::Instant;
use crate::config::EResult;
use self::basic::{
    AppContext, Command, Components, default_font, load_font, Message,
    Textures, UIStatus,
};
use self::config::Config;
use self::mouse::{make_mouse_listener, MouseEvent, MouseListener, MouseTracker, SpreadStatus};
use self::view::View;
use self::functions::{Function, FunctionBox, Functions};

mod basic;
mod view;
mod convert;
mod config;
mod ui;
mod mouse;
mod functions;

static ZOOM_IN_SCALE: Num = 1.1;
static ZOOM_OUT_SCALE: Num = 1. / ZOOM_IN_SCALE;

async fn event(mut context: AppContext, functions: &mut Functions) -> AppContext {
    for func in functions {
        if let Some(command) = func.event(&context) {
            context = handle(context, command).await;
        }
    }
    
    let command = if let Some(code) = get_last_key_pressed() {
        check_keyboard(&context, code).await
    } else if let Some(command) = context.mouse_tracker.update(()).await {
        command
    } else {
        Command::None
    };
    
    return if let Command::None = command {
        context
    } else {
        handle(context, command).await
    };
}


const SCALE_MIN: Num = 0.1;
const SCALE_MIN_RESIZE: Num = 2.;
const SCALE_MAX: Num = 0.9;
const SCALE_MAX_RESIZE: Num = 0.5;

async fn auto_zoom(mut context: AppContext) -> AppContext {
    let mut max_d = num(0);
    let width = context.view.half_width();
    let height = context.view.half_height();
    for body in context.controller.iter()
    {
        let pos = context.view.convert(*(body.pos())).0;
        let x = abs(pos.x()) / width;
        let y = abs(pos.y()) / height;
        let max = if let Ordering::Greater = x.total_cmp(&y) {
            x
        } else {
            y
        };
        if let Ordering::Greater = max.total_cmp(&max_d) {
            max_d = max;
        }
    }
    
    
    // println!("min: {}, max: {}", min_d, max_d);
    if max_d > SCALE_MAX {
        context.view.zoom(min(SCALE_MAX_RESIZE, max_d));
        // context.add_message(format!("Resize view: {}", SCALE_MAX_RESIZE).into());
        context = Box::pin(refresh(context)).await;
    } else if max_d < SCALE_MIN {
        context.view.zoom(max(SCALE_MIN_RESIZE, max_d));
        // context.add_message(format!("Resize view: {}", SCALE_MIN_RESIZE).into());
        context = Box::pin(refresh(context)).await;
    }
    
    context
}

async fn add_trail(mut context: AppContext) -> AppContext {
    let mut new_trails = context.bodies.clone();
    new_trails.iter_mut()
        .for_each(|(point, id)| {
            let body = context.controller.get_body(*id).unwrap();
            point.color = body.trail_color();
            point.color.a *= context.config.trail_alpha_begin;
            point.radius = context.config.trail_radius;
        });
    context.points.extend(new_trails.iter().map(|(point, _)| point));
    
    context
}

async fn update_trail(context: &mut AppContext) {
    let rate = context.config.trail_alpha_loss_rate;
    let min_alpha = context.config.trail_alpha_min;
    
    context.points.iter_mut().for_each(|trail| trail.color.a *= rate);
    while context.points.front().map_or(
        false,
        |trail| trail.color.a < min_alpha,
    ) {
        context.points.pop_front();
    }
}

async fn update_messages(context: &mut AppContext) {
    let rate = context.config.message_alpha_loss_rate;
    let min_alpha = context.config.message_alpha_min;
    context.messages.iter_mut().for_each(|msg| {
        if msg.delay == 0 {
            msg.color.a *= rate;
        } else {
            msg.delay -= 1;
        }
    });
    while context.messages.front().map_or(
        false,
        |msg| msg.delay == 0 && msg.color.a < min_alpha,
    ) {
        context.messages.pop_front();
    }
}

async fn draw_functions(context: &AppContext, functions: &Functions) {
    for function in functions.iter() {
        function.draw(context);
    }
}

async fn update_functions(mut context: AppContext, functions: &mut Functions) -> AppContext {
    for func in functions.iter_mut() {
        context = func.update(context);
    }
    
    context
}

async fn update(mut context: AppContext) -> AppContext {
    context.bodies = context.view.parse(&context);
    context.view.refresh(num(screen_width()), num(screen_height()));
    
    update_trail(&mut context).await;
    
    update_messages(&mut context).await;
    
    
    context
}

async fn step(mut context: AppContext) -> AppContext {
    let step_per_trail = context.config.step_per_trail;
    let (delta, steps) = match context.steps.cmp(&0) {
        Ordering::Equal => { return context; }
        Ordering::Greater => {
            (context.config.delta, context.steps)
        }
        Ordering::Less => {
            (-context.config.delta, -context.steps)
        }
    };
    
    let count = steps / step_per_trail;
    for _ in 0..count {
        for _ in 0..step_per_trail {
            context.controller.update(delta);
            context.time += delta;
        }
        context = add_trail(context).await;
    }
    
    context
}

fn prepare_delta(x: Num) -> Num {
    x / -100.
}

async fn refresh(mut context: AppContext) -> AppContext {
    context.points.clear();
    context = update(context).await;
    context
}

async fn screenshot(mut context: AppContext) -> AppContext {
    let running = context.running;
    context.running = false;
    
    ui::draw(&context, &Vec::new()).await;
    let img = get_screen_data();
    next_frame().await;
    
    draw_text_ex(
        "Saving screenshot ...",
        screen_width() / 2.0 - 20.0 * 9.5,
        screen_height() / 2.0 + 20.0 / 2.,
        TextParams {
            font_size: 40,
            font: Some(&default_font()),
            color: WHITE,
            ..Default::default()
        },
    );
    next_frame().await;
    
    let path = format!("./{}.png", chrono::Local::now().format("%Y%m%d_%H%M%S"));
    match std::panic::catch_unwind(
        || img.export_png(path.as_str())
    ) {
        Err(err) => { context.add_message(format!("Failed to save screenshot: {:?}", err).into()); }
        Ok(_) => { context.add_message(format!("Screenshot saved: {:?}", path).into()); }
    }
    context.running = running;
    
    context
}

async fn check_keyboard(context: &AppContext, code: KeyCode) -> Command {
    return match code {
        KeyCode::Escape => Command::Exit,
        KeyCode::Space => Command::TogglePause,
        KeyCode::Up => Command::AddSpeed(context.config.steps),
        KeyCode::Down => Command::AddSpeed(-context.config.steps),
        KeyCode::L => Command::AddSpeed(-2 * context.steps),
        KeyCode::U => Command::ToggleUI(Components::UI),
        KeyCode::T => Command::ToggleUI(Components::Tooltip),
        KeyCode::M => Command::ToggleUI(Components::Message),
        KeyCode::H => Command::ToggleUI(Components::Help),
        KeyCode::A => Command::ToggleUI(Components::Axis),
        KeyCode::B => Command::ToggleUI(Components::Bodies),
        KeyCode::I => Command::ToggleUI(Components::Trail),
        KeyCode::S => Command::Screenshot,
        KeyCode::P => Command::AddMessage(format!("Mouse at: {:?}", mouse_position()).into()),
        KeyCode::R => Command::ResetView,
        KeyCode::X => Command::AutoZoom,
        _ => Command::None,
    };
}

async fn handle(mut context: AppContext, command: Command) -> AppContext {
    // println!("Handle event: {:?}", command);
    match command {
        Command::RotateView(dx, dy) => {
            let dx = prepare_delta(dx);
            let dy = prepare_delta(dy);
            if dx != 0. || dy != 0. {
                context.view.rotate_view((dx, dy));
                context = refresh(context).await;
            }
        }
        Command::ZoomIn => {
            context.view.zoom(ZOOM_IN_SCALE);
            context = refresh(context).await;
        }
        Command::ZoomOut => {
            context.view.zoom(ZOOM_OUT_SCALE);
            context = refresh(context).await;
        }
        Command::ResetView => {
            context.view.reset_view();
            context = refresh(context).await;
            context.add_message("View reset".into());
        }
        Command::Exit => {
            context.exit = true;
        }
        Command::TogglePause => {
            context.running = !context.running;
        }
        Command::ToggleUI(id) => {
            context.ui_status.toggle(id);
        }
        Command::AddMessage(msg) => {
            context.add_message(msg);
        }
        Command::MultiCommand(cmds) => {
            for cmd in cmds {
                context = Box::pin(handle(context, cmd)).await;
            }
        }
        Command::Screenshot => {
            context = screenshot(context).await;
        }
        Command::AddSpeed(d) => {
            context.steps += d;
            context.add_message(format!("Steps set to: {}", context.steps).into());
        }
        Command::AutoZoom => {
            context = auto_zoom(context).await;
            context = refresh(context).await;
        }
        Command::None => {}
    }
    context
}

fn check_actions(event: MouseEvent, _: ()) -> SpreadStatus<Command> {
    match event {
        MouseEvent::Click(_) => {
            SpreadStatus::Stop(Command::TogglePause)
        }
        MouseEvent::DoubleClick(_) => {
            SpreadStatus::Stop(Command::ToggleUI(Components::UI))
        }
        MouseEvent::TripleClick(_) => {
            SpreadStatus::Stop(Command::ToggleUI(Components::Tooltip))
        }
        _ => {
            SpreadStatus::Route(None)
        }
    }
}

fn check_view_rotation(event: MouseEvent, _: ()) -> SpreadStatus<Command> {
    match event {
        MouseEvent::Dragging { diff, .. } => {
            let cmd = Command::RotateView(num(diff.0), num(diff.1));
            // println!("{:?}", cmd);
            SpreadStatus::Stop(cmd)
        }
        MouseEvent::ScrollDown(_) => {
            SpreadStatus::Stop(Command::ZoomOut)
        }
        MouseEvent::ScrollUp(_) => {
            SpreadStatus::Stop(Command::ZoomIn)
        }
        _ => {
            SpreadStatus::Route(None)
        }
    }
}

async fn init(config: Config) -> (AppContext, Functions) {
    let begin = Instant::now();
    let (mut context, functions) = init_impl(config).await;
    let duration = Instant::now().duration_since(begin);
    context.add_message(format!("Init costed {}s", duration.as_secs_f32()).into());
    
    (context, functions)
}

async fn init_impl(config: Config) -> (AppContext, Functions) {
    clear_background(BLACK);
    draw_text_ex(
        "Loading ...",
        screen_width() / 2.0 - 20.0 * 5.5,
        screen_height() / 2.0 + 20.0 / 2.,
        TextParams {
            font_size: config.title_font_size,
            font: Some(&default_font()),
            color: config.title_font_color.into(),
            ..Default::default()
        },
    );
    next_frame().await;
    
    let bodies = config.bodies.clone();
    let tooltip_font = load_font(config.tooltip_font.as_ref()).await;
    let instruction_font = if config.tooltip_font == config.instruction_font {
        tooltip_font.clone()
    } else {
        load_font(config.instruction_font.as_ref()).await
    };
    
    
    let listeners: Vec<Box<dyn MouseListener<Msg = (), Result = Command>>> = vec![
        Box::new(make_mouse_listener(check_view_rotation)),
        Box::new(make_mouse_listener(check_actions)),
    ];
    let mouse_tracker: MouseTracker<(), Command>
        = MouseTracker::from_vec(MouseButton::Left, listeners);
    
    let mut ui_status = UIStatus::default();
    ui_status.toggle(Components::Help);
    ui_status.toggle(Components::Axis);
    
    let steps = config.steps;
    
    let functions = vec![
        Box::new(functions::collision::CollisionDetect::new(&config)) as FunctionBox
    ];
    
    let textures = Textures::load();
    
    let context = AppContext {
        config,
        points: VecDeque::new(),
        messages: VecDeque::from([Message::new("Press [H] to show help".to_string())]),
        controller: Controller::new(bodies),
        running: true,
        exit: false,
        bodies: Vec::new(),
        tooltip_font,
        instruction_font,
        ui_status,
        mouse_tracker,
        view: View::new(num(screen_width()), num(screen_height())),
        textures,
        steps,
        time: 0.0,
    };
    
    (context, functions)
}

pub async fn app(config: Config) {
    let (mut context, mut functions) = init(config).await;
    
    context = auto_zoom(context).await;
    loop {
        context = event(context, &mut functions).await;
        if context.exit {
            break;
        }
        if context.running {
            context = step(context).await;
            context = update(context).await;
            context = update_functions(context, &mut functions).await;
        }
        ui::draw(&context, &functions).await;
        draw_functions(&context, &functions).await;
        next_frame().await;
    }
}

pub fn main() -> EResult<()> {
    let config: Config = crate::config::init()?;

    macroquad::Window::from_config(
        Conf {
            window_title: config.window_title.clone(),
            high_dpi: config.enable_high_dpi,
            window_width: config.window_width,
            window_height: config.window_height,
            ..Default::default()
        },
        app(config),
    );


    Ok(())
}
