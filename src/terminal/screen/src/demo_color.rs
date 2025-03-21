use std::io;
use term_system::theme::Theme;
use term_system::tui;
use term_system::window::{Screen, Window, WindowName};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;

pub struct DemoColor {
    pub window: Window,
}

const GRADIENT_LENGTH: u16 = 32;
const NUM_COLORS: usize = 8;
impl Screen for DemoColor {
    fn new(window: Window) -> Self {
        Self { window }
    }

    fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<WindowName> {
        self.window.quit = false;
        while !self.window.quit {
            terminal.draw(|frame| frame.render_widget(&mut *self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(WindowName::Demo)
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if let KeyCode::Esc = key_event.code {
            self.window.quit = true
        }
    }
}

impl Widget for &mut DemoColor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        render_theme(area, buf, Theme::AMBER);
    }
}

fn render_theme(area: Rect, buf: &mut Buffer, theme: Theme) {
    let mut constraints = vec![];
    for _ in 0..NUM_COLORS {
        constraints.push(Constraint::Length(1));
    }
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    render_color(
        layout[0],
        buf,
        theme.white_light,
        theme.white,
        theme.white_dark,
    );

    render_color(
        layout[1],
        buf,
        theme.black_light,
        theme.black,
        theme.black_dark,
    );

    render_color(layout[2], buf, theme.red_light, theme.red, theme.red_dark);

    render_color(
        layout[3],
        buf,
        theme.green_light,
        theme.green,
        theme.green_dark,
    );

    render_color(
        layout[4],
        buf,
        theme.blue_light,
        theme.blue,
        theme.blue_dark,
    );

    render_color(
        layout[5],
        buf,
        theme.cyan_light,
        theme.cyan,
        theme.cyan_dark,
    );

    render_color(
        layout[6],
        buf,
        theme.yellow_light,
        theme.yellow,
        theme.yellow_dark,
    );

    render_color(
        layout[7],
        buf,
        theme.magenta_light,
        theme.magenta,
        theme.magenta_dark,
    );
}

fn render_color(area: Rect, buf: &mut Buffer, light_color: Color, color: Color, dark_color: Color) {
    buf.cell_mut(Position {
        x: area.left(),
        y: area.top(),
    })
    .unwrap()
    .set_char('█')
    .set_fg(light_color);

    buf.cell_mut(Position {
        x: area.left() + 1,
        y: area.top(),
    })
    .unwrap()
    .set_char('█')
    .set_fg(color);

    buf.cell_mut(Position {
        x: area.left() + 2,
        y: area.top(),
    })
    .unwrap()
    .set_char('█')
    .set_fg(dark_color);

    for x in (area.left() + 4)..(GRADIENT_LENGTH + 4) {
        let color = Theme::lerp(
            light_color,
            dark_color,
            1.0 - (((x) as f32) / ((GRADIENT_LENGTH + 4) as f32)),
        );

        buf.cell_mut(Position {
            x: x,
            y: area.top(),
        })
        .unwrap()
        .set_char('█')
        .set_fg(color);
    }
}
