use std::collections::VecDeque;
use std::fmt::Debug;
use macroquad::color::{Color, WHITE};
use macroquad::prelude::load_ttf_font;
use macroquad::text::{Font, load_ttf_font_from_bytes};
use macroquad::texture::Texture2D;
use serde::{Deserialize, Serialize};
use crate::apps::app::MouseTracker;
use crate::apps::app::view::View;
use crate::body::BodyId;
use super::config::{Config, Body};
use crate::controller::Controller;
use crate::num::Num;
use crate::vector::Vector2D;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub body_id: BodyId,
    pub pos: Vector2D,
    pub color: Color,
    pub radius: Num,
    pub depth: Num,
}


#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Command {
    RotateView(Num, Num),
    ZoomIn,
    ZoomOut,
    ResetView,
    AutoZoom,
    Exit,
    TogglePause,
    Screenshot,
    AddSpeed(isize),
    ToggleUI(Components),
    AddMessage(Message),
    MultiCommand(Vec<Command>),
    None,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub content: String,
    pub color: Color,
    pub delay: usize,
}

impl Message {
    pub fn new(content: String) -> Self {
        Self { content, color: WHITE, delay: 130 }
    }
    
    #[allow(dead_code)]
    pub fn new_ex(content: String, color: Color, delay: usize) -> Self {
        Self { content, color, delay }
    }
}

impl From<String> for Message {
    fn from(content: String) -> Self {
        Self::new(content)
    }
}

impl From<&str> for Message {
    fn from(content: &str) -> Self {
        Self::new(content.to_string())
    }
}

pub struct Textures {
    pub logo: Texture2D,
    pub background: Texture2D,
}

impl Textures {
    pub fn load() -> Self {
        let logo = Texture2D::from_file_with_format(
            include_bytes!("../../res/logo.png"),
            None,
        );
        
        let background = Texture2D::from_file_with_format(
            include_bytes!("../../res/background.png"),
            None,
        );
        
        Self { logo, background }
    }
}

pub struct AppContext {
    pub config: Config,
    pub points: VecDeque<Point>,
    pub messages: VecDeque<Message>,
    pub bodies: Vec<(Point, BodyId)>,
    pub controller: Controller<Body>,
    pub exit: bool,
    pub running: bool,
    pub tooltip_font: Font,
    pub instruction_font: Font,
    pub ui_status: UIStatus,
    pub mouse_tracker: MouseTracker<(), Command>,
    pub view: View,
    pub steps: isize,
    pub time: Num,
    pub textures: Textures,
}

impl AppContext {
    pub fn add_message(&mut self, message: Message) {
        // println!("{}", message.content);
        self.messages.push_back(message);
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum OnOffStatus {
    On,
    Off,
}

impl Default for OnOffStatus {
    fn default() -> Self {
        OnOffStatus::On
    }
}

impl OnOffStatus {
    fn toggle(&mut self) -> OnOffStatus {
        let status = match self {
            OnOffStatus::On => OnOffStatus::Off,
            OnOffStatus::Off => OnOffStatus::On,
        };
        *self = status;
        status
    }
    
    fn is_on(&self) -> bool {
        match self {
            OnOffStatus::On => true,
            OnOffStatus::Off => false,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Components {
    Tooltip,
    UI,
    Help,
    Axis,
    Bodies,
    Message,
    Trail,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UIStatus {
    tooltip: OnOffStatus,
    ui: OnOffStatus,
    help: OnOffStatus,
    axis: OnOffStatus,
    bodies: OnOffStatus,
    message: OnOffStatus,
    trail: OnOffStatus,
}

impl UIStatus {
    pub fn is_on(&self, which: Components) -> bool {
        match which {
            Components::Tooltip => self.tooltip.is_on(),
            Components::UI => self.ui.is_on(),
            Components::Help => self.help.is_on(),
            Components::Axis => self.axis.is_on(),
            Components::Message => self.message.is_on(),
            Components::Bodies => self.bodies.is_on(),
            Components::Trail => self.trail.is_on(),
        }
    }
    pub fn toggle(&mut self, which: Components) -> OnOffStatus {
        match which {
            Components::Tooltip => self.tooltip.toggle(),
            Components::UI => self.ui.toggle(),
            Components::Help => self.help.toggle(),
            Components::Axis => self.axis.toggle(),
            Components::Message => self.message.toggle(),
            Components::Bodies => self.bodies.toggle(),
            Components::Trail => self.trail.toggle(),
        }
    }
}

static mut __DEFAULT_FONT: Option<Font> = None;

pub fn default_font() -> Font {
    if let Some(font) = unsafe { __DEFAULT_FONT.clone() } {
        font
    } else {
        let font = load_ttf_font_from_bytes(include_bytes!("../../res/JetBrains Mono Regular.ttf")).unwrap();
        unsafe { __DEFAULT_FONT = Some(font.clone()); }
        font
    }
}

pub async fn load_font(name: Option<&String>) -> Font {
    if let Some(font) = name {
        if let Ok(font) = load_ttf_font(font.as_str()).await {
            font
        } else {
            panic!("Font not found")
        }
    } else {
        default_font()
    }
}