use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;
use std::io;
use term_system::terminal_image;
use term_system::theme::Theme;
use term_system::tui;
use term_system::window::Screen;
use term_system::window::Window;
use term_system::window::WindowName;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;

pub struct DemoImage {
    pub window: Window,
}

impl Screen for DemoImage {
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

impl Widget for &mut DemoImage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let top_images = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
                Constraint::Percentage(10),
            ])
            .split(sections[0]);

        Paragraph::new("")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .style(Style::default().fg(Theme::AMBER.white)),
            )
            .render(sections[0], buf);

        terminal_image::render_image_path_with_border(
            top_images[0],
            buf,
            "some_fake_image",
            Theme::AMBER,
            BorderType::Thick,
        );
        terminal_image::render_image_path(
            top_images[1],
            buf,
            "asset/sprite/icon/blabber.png",
            Theme::AMBER,
        );
        terminal_image::render_image_path_with_border(
            top_images[2],
            buf,
            "asset/sprite/icon/lightning.png",
            Theme::AMBER,
            BorderType::Rounded,
        );
        terminal_image::render_image_path_with_border(
            top_images[3],
            buf,
            "asset/sprite/icon/lightning.png",
            Theme::AMBER,
            BorderType::QuadrantInside,
        );

        let bottom_images = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(40),
                Constraint::Percentage(30),
                Constraint::Percentage(20),
                Constraint::Percentage(10),
            ])
            .split(sections[1]);

        Paragraph::new("")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .style(Style::default().fg(Theme::AMBER.white)),
            )
            .render(sections[1], buf);
        terminal_image::render_image_path_with_border(
            bottom_images[0],
            buf,
            "asset/sprite/icon/cheese.png",
            Theme::AMBER,
            BorderType::Double,
        );
        terminal_image::render_image_path_with_border(
            bottom_images[2],
            buf,
            "asset/sprite/icon/cheese.png",
            Theme::AMBER,
            BorderType::Plain,
        );
        terminal_image::render_image_path_with_border(
            bottom_images[3],
            buf,
            "asset/sprite/icon/cheese.png",
            Theme::AMBER,
            BorderType::Rounded,
        );
        let x = ["TEXT"];
        let y: Vec<_> = x.iter().cycle().take(x.len() * 200).collect();
        Paragraph::new(format!("{:?}", y))
            .wrap(Wrap { trim: true })
            .block(Block::new())
            .bg(Theme::AMBER.black_dark)
            .fg(Theme::AMBER.white)
            .render(bottom_images[1], buf);
    }
}
