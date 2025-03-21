use std::{fmt, io};

use crate::{theme::Theme, tui};
use crossterm::event::KeyEvent;
use ratatui::widgets::block::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum WindowName {
    None,

    Menu,

    Database,
    Arkachat,
    Options,
    Demo,

    DemoColor,
    DemoImage,
}

#[derive(Copy, Clone)]
pub struct Window {
    pub quit: bool,
    pub change: bool,
    pub theme: Theme,
    pub border_type: BorderType,
}

pub trait Screen {
    fn new(window: Window) -> Self;
    fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<WindowName>;
    fn handle_events(&mut self) -> io::Result<()>;
    fn handle_key_event(&mut self, key_event: KeyEvent);
}

impl Default for Window {
    fn default() -> Window {
        Window {
            quit: false,
            change: false,
            theme: Theme::AMBER,
            border_type: BorderType::Rounded,
        }
    }
}

impl fmt::Display for WindowName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
