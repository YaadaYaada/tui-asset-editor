use std::io;
use term_system::theme::Theme;
use term_system::tui;
use term_system::window::{Screen, Window, WindowName};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

struct MenuOption<'a> {
    menu_option: &'a WindowName,
}

struct MenuOptionList<'a> {
    state: ListState,
    menu_options: Vec<MenuOption<'a>>,
}

pub struct Menu<'a> {
    window: Window,
    menu_options: MenuOptionList<'a>,
}

impl<'a> Screen for Menu<'a> {
    fn new(window: Window) -> Self {
        Self {
            window,
            menu_options: MenuOptionList::with_menu_options(vec![
                &WindowName::Database,
                &WindowName::Arkachat,
                &WindowName::Options,
                &WindowName::Demo,
            ]),
        }
    }

    fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<WindowName> {
        self.window.quit = false;
        self.window.change = false;

        if self.menu_options.state.selected().is_none() {
            self.menu_options.state.select(Some(0));
        }

        while !self.window.quit && !self.window.change {
            terminal.draw(|frame| frame.render_widget(&mut *self, frame.area()))?;
            self.handle_events()?;
        }
        if self.window.change {
            return Ok(*self.menu_options.menu_options
                [self.menu_options.state.selected().unwrap()]
            .menu_option);
        }
        Ok(WindowName::None)
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
            KeyCode::Up => self.menu_options.previous(),
            KeyCode::Down => self.menu_options.next(),
            _ => {}
        }
    }
}

impl<'a> Widget for &mut Menu<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let half_width = area.width / 2;
        let half_height = area.height / 2;
        let horizontal_sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(half_height - 1),
                // Options box height
                Constraint::Length(6),
                Constraint::Length(half_height - 5),
            ])
            .split(area);

        let title_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(half_width.saturating_sub(31)),
                // Title width
                Constraint::Length(62),
                Constraint::Length(half_width.saturating_sub(31)),
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
        render_menu(options_layout[1], buf, &mut self.menu_options, self.window);
    }
}

fn render_title(area: Rect, buf: &mut Buffer, window: Window) {
    let title = "
   █████╗ ██████╗ ██╗  ██╗ █████╗ ███╗   ██╗███████╗████████╗
  ██╔══██╗██╔══██╗██║ ██╔╝██╔══██╗████╗  ██║██╔════╝╚══██╔══╝
  ███████║██████╔╝█████╔╝ ███████║██╔██╗ ██║█████╗     ██║
  ██╔══██║██╔══██╗██╔═██╗ ██╔══██║██║╚██╗██║██╔══╝     ██║
  ██║  ██║██║  ██║██║  ██╗██║  ██║██║ ╚████║███████╗   ██║
  ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝   ╚═╝
█████████████████████████████████████████████████████████████╗
╚════════════════════════════════════════════════════════════╝
";

    // Spacing between title and top of terminal
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
                window.theme.white,
                window.theme.white_dark,
                1.0 - (i as f32) / (title_lines.len() as f32),
            ))
            .render(title_layout[i], buf);
    }
}

fn render_menu(area: Rect, buf: &mut Buffer, menu_options: &mut MenuOptionList, window: Window) {
    let list_items: Vec<ListItem> = menu_options
        .menu_options
        .iter()
        .map(|menu_option| menu_option.to_list_item())
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(window.border_type)
                .style(Style::default().fg(window.theme.white)),
        )
        .style(Style::default().fg(window.theme.white))
        .highlight_style(
            Style::default()
                .fg(window.theme.black_dark)
                .bg(window.theme.white),
        )
        .direction(ListDirection::TopToBottom);
    StatefulWidget::render(list, area, buf, &mut menu_options.state);
}

impl MenuOptionList<'_> {
    fn with_menu_options(menu_options: Vec<&WindowName>) -> MenuOptionList {
        MenuOptionList {
            state: ListState::default(),
            menu_options: menu_options.iter().map(MenuOption::from).collect(),
        }
    }

    fn next(&mut self) {
        if self.state.selected().unwrap() < self.menu_options.len() - 1 {
            self.state.select(Some(self.state.selected().unwrap() + 1));
        }
    }

    fn previous(&mut self) {
        if self.state.selected().unwrap() > 0 {
            self.state.select(Some(self.state.selected().unwrap() - 1));
        }
    }
}

impl<'a> From<&&'a WindowName> for MenuOption<'a> {
    fn from(menu_option: &&'a WindowName) -> Self {
        Self { menu_option }
    }
}

impl MenuOption<'_> {
    fn to_list_item(&self) -> ListItem {
        ListItem::new(self.menu_option.to_string())
    }
}
