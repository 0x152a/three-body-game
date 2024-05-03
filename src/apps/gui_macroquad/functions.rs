pub mod collision;

use crate::body::BodyId;
use macroquad::prelude::*;
use crate::body::BodyLike;
use crate::controller::IterStatus;
use crate::num::{Num, num};
use super::basic::{AppContext, default_font, Command};
use super::config::{Config, AttrValue, Body};
#[allow(unused_imports)]
use super::ui::{draw_mask, draw_3d_point, draw_3d_arrow, draw_3d_line};

pub type Functions = Vec<FunctionBox>;

#[allow(unused_variables)]
pub trait Function {
    fn new(config: &Config) -> Self
        where
            Self: Sized;
    #[allow(unused_mut)]
    fn update(&mut self, mut context: AppContext) -> AppContext { context }
    fn draw(&self, context: &AppContext) {}
    fn event(&mut self, context: &AppContext) -> Option<Command> { None }
    fn make_help(&self, context: &AppContext, help: &mut Vec<(String, String)>) {}
    fn make_tooltip(&self, context: &AppContext, tooltip: &mut Vec<String>) {}
    fn make_title(&self, context: &AppContext, title: &mut Vec<String>) {}
}

pub type FunctionBox = Box<dyn Function>;

