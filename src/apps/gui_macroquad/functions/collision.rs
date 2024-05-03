use crate::apps::app::basic::Command::AddMessage;
use super::*;

#[derive(Debug, Copy, Clone)]
enum Status {
    None,
    Detected(BodyId, BodyId),
}


#[derive(Debug, Copy, Clone)]
pub struct CollisionDetect {
    status: Status,
    on: bool,
}

impl Default for CollisionDetect {
    fn default() -> Self {
        Self {
            status: Status::None,
            on: true,
        }
    }
}

static mut __DEFAULT_DENSITY: Option<Num> = None;

fn get_density(context: &AppContext, this: &Body) -> Num {
    if let Some(AttrValue::Num(x)) = this.get_attr(&"density".into()) {
        x
    } else if let Some(x) = unsafe { __DEFAULT_DENSITY } {
        x
    } else if let Some(AttrValue::Num(x)) = context.config.fields.get(&"default_density".to_string()) {
        let x = *x;
        unsafe { __DEFAULT_DENSITY = Some(x) }
        x
    } else {
        num(10)
    }
}

fn collides_with(context: &AppContext, this: &Body, other: &Body) -> bool {
    this.pos().distance(*other.pos()) <=
        this.mass() / get_density(context, this) + other.mass() / get_density(context, other)
}


#[allow(unused_variables)]
impl Function for CollisionDetect {
    fn new(config: &Config) -> Self {
        Default::default()
    }
    
    fn update(&mut self, mut context: AppContext) -> AppContext {
        if !self.on {
            return context;
        }
        
        if let Status::Detected(..) = self.status {
            if context.running {
                // Forced
                self.status = Status::None;
            }
            return context;
        }
        
        let result = context.controller.with_each_other(
            |this, other| {
                return if collides_with(&context, this, other) {
                    IterStatus::Done((*this.id(), *other.id()))
                } else {
                    IterStatus::Continue
                };
            }
        );
        
        if let Some((this, other)) = result {
            context.add_message(
                format!(
                    "Body {} and {} collided at {:.3}",
                    this, other, context.time
                ).into()
            );
            context.running = false;
            self.status = Status::Detected(this, other);
        } else {
            self.status = Status::None;
        }
        
        context
    }
    
    fn draw(&self, context: &AppContext) {
        if let Status::Detected(this, other) = &self.status {
            draw_mask(BLACK);
            
            draw_text_ex(
                "COLLISION",
                screen_width() / 2.0 - 20.0 * 5.5,
                screen_height() / 2.0 + 20.0 / 2.,
                TextParams {
                    font_size: context.config.title_font_size,
                    font: Some(&default_font()),
                    color: context.config.title_font_color.into(),
                    ..Default::default()
                },
            );
        }
    }
    
    fn event(&mut self, context: &AppContext) -> Option<Command> {
        if is_key_released(KeyCode::C) {
            self.on = !self.on;
            Some(AddMessage(
                format!("Collision Detect turned {}", if self.on { "on" } else { "off" }).into()
            ))
        } else {
            None
        }
    }
    
    fn make_help(&self, context: &AppContext, help: &mut Vec<(String, String)>) {
        help.push((
            "C".into(),
            format!("{} Collision Detect", if self.on { "Turn off" } else { "Turn on" })
        ))
    }
}
