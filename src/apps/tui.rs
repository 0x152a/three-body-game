use color_eyre::Result as EResult;
use crate::controller::Controller;
use std::io::{self, stdout, Stdout};
use crossterm::{
    execute,
    terminal::*,
    event::{self, Event, KeyEventKind, KeyCode, KeyEvent},
    cursor,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use ui::UI;

mod convert;
mod view;
mod ui;

type Tui = Terminal<CrosstermBackend<Stdout>>;

pub struct Guard;

impl Guard {
    pub fn init(&self) -> io::Result<Tui> {
        execute!(
            stdout(),
            EnterAlternateScreen,
            event::EnableMouseCapture,
            cursor::Hide,
            crossterm::style::Print(include_str!("../res/banner.txt")),
        )?;
        enable_raw_mode()?;
        Terminal::new(CrosstermBackend::new(stdout()))
    }
    
    pub fn restore(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(
            stdout(),
            LeaveAlternateScreen,
            crossterm::style::ResetColor,
            cursor::Show,
        )?;
        Ok(())
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        self.restore().expect("Unable to restore terminal");
    }
}

pub struct App {
    controller: Controller,
    exit: bool,
}

impl App {
    pub fn new(controller: Controller) -> Self {
        Self {
            controller,
            exit: false,
        }
    }
    
    pub fn run(&mut self) -> EResult<()> {
        let guard = Guard;
        let mut terminal = guard.init()?;
        let mut ui = UI::new(&self.controller);
        while !self.exit {
            terminal.draw(|frame| ui.render(frame))?;
            ui.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }
    
    fn exit(&mut self) {
        self.exit = true;
    }
}
