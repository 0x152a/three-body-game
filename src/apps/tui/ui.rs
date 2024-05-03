use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::Alignment;
use ratatui::prelude::*;
use ratatui::prelude::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::widgets::canvas::{Canvas, Context};
use super::view::View;
use crate::controller::Controller;
use color_eyre::{
    eyre::WrapErr, Result as EResult,
};

enum UIStatus {
    Pause,
    Running,
}


pub struct UI<'a> {
    controller: &'a Controller,
    ui_status: UIStatus,
    view: View,
}

impl<'a> UI<'a> {
    pub fn new(controller: &'a Controller) -> Self {
        Self {
            controller,
            ui_status: UIStatus::Pause,
            view: View::new(360, 360),
        }
    }
    
    fn render_hint(&self, frame: &mut Frame, area: Rect) {
        let time = self.controller.total_seconds().to_string();
        let instructions = Paragraph::new(if let UIStatus::Running = self.ui_status {
            vec![
                " Pause ".into(),
                Line::from("<Space>").style(Style::new().blue()),
                " Quit ".into(),
                Line::from("<Q>").style(Style::new().blue()),
                " Time ".into(),
                Line::from(time).style(Style::new().blue()),
            ]
        } else {
            vec![
                " Resume ".into(),
                Line::from("<Space>").style(Style::new().blue()),
                " Quit ".into(),
                Line::from("<Q>").style(Style::new().blue()),
                " Time ".into(),
                Line::from(time).style(Style::new().blue()),
            ]
        });
        let instructions = instructions.alignment(Alignment::Center);
        frame.render_widget(instructions, area);
    }
    fn render_icon(&self, frame: &mut Frame, area: Rect) {
        let icon = Paragraph::new(include_str!("../../res/icon.txt"))
            .style(Style::new().bg(Color::Rgb(140, 149, 155)).fg(Color::Rgb(62, 123, 186)));
        frame.render_widget(icon, area);
    }
    
    fn paint_board(&self, ctx: &mut Context) {
        ctx.draw(&ratatui::widgets::canvas::Line {
            x1: 0.0,
            y1: 10.0,
            x2: 10.0,
            y2: -10.0,
            color: Color::Cyan,
        });
        for bc in self.view.parse(self.controller) {
            ctx.layer();
            ctx.draw(&ratatui::widgets::canvas::Circle {
                x: bc.pos.x(),
                y: bc.pos.y(),
                radius: 2.0,
                color: Color::White,
            });
        }
    }
    
    fn render_board(&self, frame: &mut Frame, area: Rect) {
        let canvas = Canvas::default()
            .x_bounds([-180.0, 180.0])
            .y_bounds([-180.0, 180.0])
            .paint(|ctx| self.paint_board(ctx));
        frame.render_widget(canvas, area);
    }
    
    pub fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        if area.width > area.height {
            // Horizontal layout
            let main_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(70),  // board
                    Constraint::Min(25),         // hind part
                ].into_iter())
                .split(area);
            let hint_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Min(20),  // icon
                    Constraint::Min(8),   // hint
                ])
                .split(main_layout[1]);
            self.render_board(frame, main_layout[0]);
            self.render_icon(frame, hint_layout[0]);
            self.render_hint(frame, hint_layout[1]);
        } else {
            // Vertical layout
            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(70),  // board
                    Constraint::Min(25),         // hind part
                ].into_iter())
                .split(area);
            let hint_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Min(20),  // icon
                    Constraint::Min(8),   // hint
                ])
                .split(main_layout[1]);
            self.render_board(frame, main_layout[0]);
            self.render_icon(frame, hint_layout[0]);
            self.render_hint(frame, hint_layout[1]);
        }
    }
    
    
    pub fn handle_events(&mut self) -> EResult<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event).wrap_err_with(|| {
                    format!("handling key event failed:\n{key_event:#?}")
                })
            }
            _ => Ok(()),
        }
    }
    
    fn handle_key_event(&mut self, key_event: KeyEvent) -> EResult<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
        Ok(())
    }
    
}