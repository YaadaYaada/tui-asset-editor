use std::io;
use term_system::theme::Theme;
use term_system::tui;
use term_system::window::{Screen, Window, WindowName};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

struct DemoOption<'a> {
    demo_option: &'a WindowName,
}

struct DemoOptionList<'a> {
    state: ListState,
    demo_options: Vec<DemoOption<'a>>,
}

pub struct Demo<'a> {
    window: Window,
    demo_options: DemoOptionList<'a>,
}

impl<'a> Screen for Demo<'a> {
    fn new(window: Window) -> Self {
        Self {
            window,
            demo_options: DemoOptionList::with_demo_options(vec![
                &WindowName::DemoColor,
                &WindowName::DemoImage,
            ]),
        }
    }

    fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<WindowName> {
        match self.demo_options.state.selected() {
            Some(_) => {}
            None => self.demo_options.state.select(Some(0)),
        }
        self.window.quit = false;
        self.window.change = false;
        while !self.window.quit && !self.window.change {
            terminal.draw(|frame| frame.render_widget(&mut *self, frame.area()))?;
            self.handle_events()?;
        }
        if self.window.change {
            return Ok(*self.demo_options.demo_options
                [self.demo_options.state.selected().unwrap()]
            .demo_option);
        }
        Ok(WindowName::Menu)
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
        match key_event.code {
            KeyCode::Esc => self.window.quit = true,
            KeyCode::Enter => self.window.change = true,
            KeyCode::Up => self.demo_options.previous(),
            KeyCode::Down => self.demo_options.next(),
            _ => {}
        }
    }
}

impl<'a> Widget for &mut Demo<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let half_width = area.width / 2;
        let half_height = area.height / 2;
        let horizontal_sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(half_height - 1),
                // Options box height
                Constraint::Length(4),
                Constraint::Length(half_height - 5),
            ])
            .split(area);

        let title_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(half_width.saturating_sub(24)),
                // Title Width
                Constraint::Length(49),
                Constraint::Length(half_width.saturating_sub(25)),
            ])
            .split(horizontal_sections[0]);

        let options_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(half_width.saturating_sub(10)),
                // Options box width
                Constraint::Length(20),
                Constraint::Length(half_width.saturating_sub(10)),
            ])
            .split(horizontal_sections[1]);

        render_title(title_layout[1], buf, self.window);
        render_demo(options_layout[1], buf, &mut self.demo_options, self.window);
    }
}

fn render_title(area: Rect, buf: &mut Buffer, window: Window) {
    let title = "
  ██████╗ ███████╗███╗   ███╗ ██████╗ ███████╗
  ██╔══██╗██╔════╝████╗ ████║██╔═══██╗██╔════╝
  ██║  ██║█████╗  ██╔████╔██║██║   ██║███████╗
  ██║  ██║██╔══╝  ██║╚██╔╝██║██║   ██║╚════██║
  ██████╔╝███████╗██║ ╚═╝ ██║╚██████╔╝███████║
  ╚═════╝ ╚══════╝╚═╝     ╚═╝ ╚═════╝ ╚══════╝
████████████████████████████████████████████████╗
╚═══════════════════════════════════════════════╝
";

    let mut constraints = vec![Constraint::Length(2)];
    let title_lines = title.split("\n").collect::<Vec<&str>>();
    for _ in 1..title_lines.len() {
        constraints.push(Constraint::Length(1));
    }
    let title_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for i in 0..title_lines.len() {
        Paragraph::new(title_lines[i])
            .block(Block::new())
            .bg(window.theme.black_dark)
            .fg(Theme::lerp(
                window.theme.blue_light,
                window.theme.blue_dark,
                1.0 - (i as f32) / (title_lines.len() as f32),
            ))
            .render(title_layout[i], buf);
    }
}

fn render_demo(area: Rect, buf: &mut Buffer, demo_options: &mut DemoOptionList, window: Window) {
    let list_items: Vec<ListItem> = demo_options
        .demo_options
        .iter()
        .map(|demo_option| demo_option.to_list_item())
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(window.border_type)
                .style(Style::default().fg(window.theme.blue)),
        )
        .style(Style::default().fg(window.theme.blue))
        .highlight_style(
            Style::default()
                .fg(window.theme.black_dark)
                .bg(window.theme.blue),
        )
        .direction(ListDirection::TopToBottom);
    StatefulWidget::render(list, area, buf, &mut demo_options.state);
}

impl DemoOptionList<'_> {
    fn with_demo_options(demo_options: Vec<&WindowName>) -> DemoOptionList {
        DemoOptionList {
            state: ListState::default(),
            demo_options: demo_options.iter().map(DemoOption::from).collect(),
        }
    }

    fn next(&mut self) {
        if self.state.selected().unwrap() < self.demo_options.len() - 1 {
            self.state.select(Some(self.state.selected().unwrap() + 1));
        }
    }

    fn previous(&mut self) {
        if self.state.selected().unwrap() > 0 {
            self.state.select(Some(self.state.selected().unwrap() - 1));
        }
    }
}

impl<'a> From<&&'a WindowName> for DemoOption<'a> {
    fn from(demo_option: &&'a WindowName) -> Self {
        Self { demo_option }
    }
}

impl DemoOption<'_> {
    fn to_list_item(&self) -> ListItem {
        ListItem::new(self.demo_option.to_string())
    }
}
