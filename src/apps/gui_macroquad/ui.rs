use std::collections::HashMap;
use macroquad::prelude::*;
use std::default::Default;
use crate::body::{BodyId, BodyLike};
use super::basic::{AppContext, Components, Message};
use crate::num::{Num, PI, num};
use crate::vector::Vector;
use super::functions::Functions;


fn to_f32(x: Num) -> f32 {
    x as f32
}



macro_rules! draw_text_lines_ex_wrap {
    ($txt: expr, $pos_x: expr, $pos_y: expr, $step: expr, $params: expr) => {
        let __len = $txt.len();
        if __len <= 1 {
            for __line in $txt {
                draw_text_ex(
                    __line.as_str(),
                    $pos_x,
                    $pos_y,
                    $params,
                );
            }
        } else {
            let mut __i = -(__len as f32) / 2.0;
            let __length = $step / 2.0;
            let __step = 2.0;
            for __line in $txt {
                draw_text_ex(
                    __line.as_str(),
                    $pos_x,
                    $pos_y + (__i * __length),
                    $params,
                );
                __i += __step;
            }
        }
    }
}


macro_rules! draw_text_lines_ex_up {
    ($txt: expr, $pos_x: expr, $pos_y: expr, $step: expr, $params: expr) => {
        let __len = $txt.len();
        if __len <= 1 {
            for __line in $txt {
                draw_text_ex(
                    __line.as_str(),
                    $pos_x,
                    $pos_y,
                    $params,
                );
            }
        } else {
            let mut __i = -(__len as f32);
            let __length = $step;
            let __step = 1.0;
            for __line in $txt {
                draw_text_ex(
                    __line.as_str(),
                    $pos_x,
                    $pos_y + (__i * __length),
                    $params,
                );
                __i += __step;
            }
        }
    }
}

#[allow(unused_macros)]
macro_rules! draw_text_lines_ex_down {
    ($txt: expr, $pos_x: expr, $pos_y: expr, $step: expr, $params: expr) => {
        let __len = $txt.len();
        if __len <= 1 {
            for __line in $txt {
                draw_text_ex(
                    __line.as_str(),
                    $pos_x,
                    $pos_y,
                    $params,
                );
            }
        } else {
            let mut __i = 0.;
            let __length = $step;
            let __step = 1.0;
            for __line in $txt {
                draw_text_ex(
                    __line.as_str(),
                    $pos_x,
                    $pos_y + (__i * __length),
                    $params,
                );
                __i += __step;
            }
        }
    }
}



pub async fn draw(context: &AppContext, functions: &Functions) {
    draw_background(&context);
    
    if context.ui_status.is_on(Components::Axis) {
        draw_axis(context).await;
    }
    
    draw_points(context).await;
    
    draw_ui(context, functions).await;
}

fn draw_background(context: &AppContext) {
    // clear_background(BLACK);
    draw_texture_ex(
        &context.textures.background,
        0., 0., WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2 { x: screen_width(), y: screen_height() }),
            ..Default::default()
        },
    );
    draw_mask_ex(Color { a: 0.3, ..BLACK });
}

#[allow(dead_code)]
pub async fn draw_3d_line(
    context: &AppContext, from: Vector, to: Vector, thickness: f32, color: Color,
) {
    let view = &context.view;
    let center = view.center();
    let from = view.convert(from).0 + center;
    let to = view.convert(to).0 + center;
    draw_line(
        to_f32(from.x()),
        to_f32(from.y()),
        to_f32(to.x()),
        to_f32(to.y()),
        thickness,
        color,
    );
}

pub async fn draw_3d_arrow(
    context: &AppContext, from: Vector, to: Vector, thickness: f32, color: Color,
) {
    let view = &context.view;
    let center = view.center();
    let from = view.convert(from).0 + center;
    let to_base = view.convert(to).0;
    let size = context.config.arrow_size;
    let to = to_base + center;
    let a = to - (to_base + to_base.rotate(PI / num(4))).unit() * size;
    let b = to - (to_base + to_base.rotate(-PI / num(4))).unit() * size;
    for target in [from, a, b] {
        draw_line(
            to_f32(target.x()),
            to_f32(target.y()),
            to_f32(to.x()),
            to_f32(to.y()),
            thickness,
            color,
        );
    }
}

pub async fn draw_3d_point(
    context: &AppContext, pos: Vector, r: f32, color: Color,
) {
    let view = &context.view;
    let pos = view.convert(pos).0;
    draw_circle(
        to_f32(pos.x() + view.half_width()),
        to_f32(pos.y() + view.half_height()),
        r,
        color,
    );
}

pub fn draw_mask(color: Color) {
    draw_rectangle(0., 0., screen_width(), screen_height(), Color { a: 0.4 * color.a, ..color });
}

pub fn draw_mask_ex(color: Color) {
    draw_rectangle(0., 0., screen_width(), screen_height(), color);
}

static AXIS: [((Num, Num, Num), Color); 3] = [
    ((1.4, 0., 0.), Color { a: 0.8, ..RED }),
    ((0., 1.4, 0.), Color { a: 0.8, ..GREEN }),
    ((0., 0., 1.4), Color { a: 0.8, ..BLUE }),
];

static AXIS_POINTS: [((Num, Num, Num), Color); 0] = [
    // ((1., 0., 0.), RED),
    // ((0., 1., 0.), GREEN),
    // ((0., 0., 1.), BLUE),
];

async fn draw_axis(context: &AppContext) {
    // Give up ...
    let view = &context.view;
    draw_circle(
        to_f32(view.half_width()),
        to_f32(view.half_height()),
        1.,
        GRAY,
    );
    draw_3d_point(context, Vector::origin(), 2., GRAY).await;
    for (pos, color) in AXIS {
        let pos = Vector::from_tuple(pos);
        draw_3d_arrow(context, Vector::origin(), pos, 1., color).await;
    }
    for (pos, color) in AXIS_POINTS {
        let pos = Vector::from_tuple(pos);
        draw_3d_point(context, pos, 2., color).await;
    }
}

async fn draw_trails(context: &AppContext) {
    // for point in &context.points {
    //     draw_circle(
    //         to_f32(context.view.get_x(point)),
    //         to_f32(context.view.get_y(point)),
    //         to_f32(point.radius),
    //         point.color,
    //     );
    // }
    let mut last_store: HashMap<BodyId, (f32, f32)> = HashMap::new();
    for point in context.points.iter().skip(1) {
        let this = (to_f32(context.view.get_x(point)), to_f32(context.view.get_y(point)));
        match last_store.get(&point.body_id) {
            Some(last) => {
                draw_line(
                    last.0, last.1, this.0, this.1,
                    to_f32(point.radius * 2.), point.color,
                );
            }
            None => {
                // draw_circle(
                //     this.0, this.1, to_f32(point.radius), point.color,
                // );
            }
        };
        last_store.insert(point.body_id, this);
    }
}

async fn draw_bodies(context: &AppContext) {
    for (body, _) in &context.bodies {
        let rate = context.config.shine_alpha_loss_rate;
        let mut r = to_f32(body.radius);
        let mut a = body.color.a * 0.1;
        while a > context.config.shine_alpha_min {
            draw_circle(
                to_f32(context.view.get_x(body)),
                to_f32(context.view.get_y(body)),
                r,
                Color {
                    a,
                    ..body.color
                },
            );
            a *= rate;
            r += 0.8;
        }
    }
}

async fn draw_points(context: &AppContext) {
    if context.ui_status.is_on(Components::Trail) {
        draw_trails(context).await;
    }
    if context.ui_status.is_on(Components::Bodies) {
        draw_bodies(context).await;
    }
}


async fn draw_ui(context: &AppContext, functions: &Functions) {
    if context.ui_status.is_on(Components::Tooltip) {
        let tooltip_font = Some(&context.tooltip_font);
        for (point, id) in context.bodies.iter() {
            let body = context.controller.get_body(*id).unwrap();
            let mut txt = vec![
                // format!(" m: {:.2}", body.mass()),
                format!("id: {:.2}", body.id()),
                format!(" v: {:.2}", body.speed().module()),
            ];
            for func in functions {
                func.make_tooltip(context, &mut txt);
            }
            draw_text_lines_ex_wrap!(
                txt,
                to_f32(context.view.get_x(point)) + 15.0,
                to_f32(context.view.get_y(point)),
                15.0,
                TextParams {
                    font: tooltip_font,
                    font_size: context.config.tooltip_font_size,
                    color: context.config.tooltip_font_color.into(),
                    ..Default::default()
                }
            );
        }
    }
    
    let instruction_font = Some(&context.instruction_font);
    let instruction_font_size = context.config.instruction_font_size;
    if context.ui_status.is_on(Components::Help) {
        // Left Bottom
        let mut instructions: Vec<(String, String)> = vec![
            ("SPACE".into(), if context.running { "Pause" } else { "Resume" }.into()),
            ("ESC".into(), "Exit".into()),
            ("Up/Down".into(), "Chane Speed".into()),
            ("R".into(), "Reset View".into()),
            ("X".into(), "Auto Zoom".into()),
            ("L".into(), format!("{} Mode", if context.steps > 0 { "Past" } else { "Present" })),
            ("S".into(), "Take Screenshot".into()),
            ("U".into(), "Toggle UI".into()),
            ("H".into(), "Toggle Help".into()),
            ("T".into(), "Toggle Tooltip".into()),
            ("A".into(), "Toggle Axis".into()),
            ("B".into(), "Toggle Bodies".into()),
            ("T".into(), "Toggle Trail".into()),
            ("M".into(), "Toggle Message".into()),
        ];
        for func in functions {
            func.make_help(context, &mut instructions);
        }
        let key_width = instructions
            .iter()
            .map(|x| x.0.len())
            .max().unwrap_or(1)
            + 2;
        let instructions = instructions
            .iter()
            .map(|(k, v)| format!("[{k:^key_width$}]: {v}"));
        draw_text_lines_ex_up!(
            instructions,
            10.0,
            screen_height(),
            instruction_font_size as f32 + 2.0,
            TextParams {
                font: instruction_font,
                font_size: instruction_font_size,
                color: context.config.instruction_font_color.into(),
                ..Default::default()
            }
        );
    }
    
    if context.ui_status.is_on(Components::Message) {
        // Right Bottom
        let len = context.messages.len();
        let mut i = -(len as f32);
        let length = instruction_font_size as f32 + 2.0;
        let step = 1.0;
        for Message { content, color, .. } in &context.messages {
            draw_text_ex(
                content,
                screen_width() - instruction_font_size as f32 * content.len() as f32 / 1.5,
                screen_height() + i * length,
                TextParams {
                    font: instruction_font,
                    font_size: instruction_font_size,
                    color: *color,
                    ..Default::default()
                },
            );
            i += step;
        }
    }
    
    if context.ui_status.is_on(Components::UI) {
        // Left Up
        let mut titles = vec![
            "".to_string(),
            format!("time: {:.3}", context.time),
            format!("body: {}", context.bodies.len()),
            // format!("scale: {:.2}", context.view.unit_size()),
        ];
        for func in functions {
            func.make_title(context, &mut titles);
        }
        draw_text_lines_ex_down!(
            titles,
            10.,
            0.,
            instruction_font_size as f32 + 2.0,
            TextParams {
                font: instruction_font,
                font_size: instruction_font_size,
                color: context.config.instruction_font_color.into(),
                ..Default::default()
            }
        );
    }
}


