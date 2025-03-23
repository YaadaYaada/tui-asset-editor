use game_mechanic::item::equipment::EquipmentSlot;
use game_mechanic::prelude::*;
use game_system::asset::asset_lib::AssetLib;
use game_system::prelude::AssetType;
use image::DynamicImage;
use ratatui::widgets::{
    Block, Borders, List, ListDirection, ListItem, ListState, Paragraph, Scrollbar,
    ScrollbarOrientation, ScrollbarState, Wrap,
};

use std::cmp::min;
use std::fmt::Display;
use std::io;
use std::rc::Rc;
use term_system::terminal_image::{load_image, set_background_color, UNKNOWN_IMAGE_PATH};
use term_system::window::{Screen, Window, WindowName};
use term_system::{terminal_image, tui};

use bevy_reflect::{GetPath, PartialReflect, Reflect, Struct};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::prelude::*;

const MAGIC_CURSOR_SYMBOL: &str = "ඞ";

#[derive(Clone, Reflect)]
struct Asset {
    name: String,
    id: u32,
    asset_type: AssetType,
    icon: String,
}

struct AssetList {
    state: ListState,
    assets: Vec<usize>,
}

pub struct Database {
    window: Window,
    // All assets from each asset lib.
    assets: Vec<Asset>,
    // All assets that match the current search.
    // What shows up in the assets frame
    visible_assets: AssetList,
    // Aura assets
    aura_lib: AuraLib,
    // Item assets
    item_lib: ItemLib,
    // The currently selected frame
    active_frame: DatabaseFrame,
    // How many lines have been scrolled in the details frame.
    details_scroll: u16,
    // How much you can scroll in the details frame.
    max_details_scroll: u16,
    // Visible text in the search frame.
    search_input: String,
    // Current character the cursor is at in the search input
    search_character_index: usize,
    // Visible text for the field being edited
    details_input: String,
    // Current character the cursor is at in the details input
    details_character_index: usize,
    // Which detail is currently selected
    details_index: usize,
    // Whether a detail is being edited
    editing_details: bool,
    // The asset currently visible in the details frame
    current_asset: Asset,
    // All of the field names belonging to the current asset
    current_asset_fields: Vec<String>,
    // The global (x,y) position of the cursor.
    cursor_position: Position,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum DatabaseFrame {
    Search,
    Assets,
    Details,
}

impl Screen for Database {
    fn new(window: Window) -> Self {
        // TODO: Make asset lib loading and asset rendering more generic.
        let aura_lib = AuraLib::new("asset/def/aura.ron");
        let item_lib = ItemLib::new("asset/def/item.ron");
        let mut assets: Vec<Asset> = vec![];
        for def in &aura_lib.defs {
            assets.push(Asset {
                name: def.name.clone(),
                id: def.id,
                asset_type: AssetType::Aura,
                icon: def.icon.clone(),
            })
        }
        for def in &item_lib.defs {
            assets.push(Asset {
                name: def.name.clone(),
                id: def.id,
                asset_type: AssetType::Item,
                icon: def.icon.clone(),
            })
        }
        let current_asset = assets[0].clone();
        let num_assets = assets.len();
        let visible_assets = AssetList::from_assets((0..num_assets).collect());
        Self {
            window,
            aura_lib,
            item_lib,
            assets,
            visible_assets,
            active_frame: DatabaseFrame::Search,
            details_scroll: 0,
            max_details_scroll: 0,
            search_input: String::new(),
            search_character_index: 0,
            details_input: String::new(),
            details_character_index: 0,
            details_index: 0,
            editing_details: false,
            current_asset,
            current_asset_fields: vec![],
            cursor_position: Position { x: 1, y: 1 },
        }
    }

    fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<WindowName> {
        self.window.quit = false;
        self.window.change = false;
        self.window.draw_background = true;

        if self.visible_assets.state.selected().is_none() {
            self.visible_assets.state.select(Some(0));
        }
        while !self.window.quit {
            let _ = terminal.draw(|frame| {
                if self.window.draw_background {
                    set_background_color(
                        frame.area(),
                        frame.buffer_mut(),
                        self.window.theme.black_dark,
                    );
                } else {
                    self.window.draw_background = false
                }

                frame.render_widget(&mut *self, frame.area());
                frame.set_cursor_position(self.cursor_position);
            });
            let _ = self.handle_events();
        }
        self.aura_lib.save("asset/def/aura.ron");
        self.item_lib.save("asset/def/item.ron");
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
        // Window wide hotkeys
        match key_event.code {
            KeyCode::Esc => self.window.quit = true,
            KeyCode::Tab => {
                self.active_frame = match self.active_frame {
                    DatabaseFrame::Search => DatabaseFrame::Assets,
                    DatabaseFrame::Assets => DatabaseFrame::Details,
                    DatabaseFrame::Details => DatabaseFrame::Search,
                }
            }
            _ => {}
        }

        match self.active_frame {
            DatabaseFrame::Search => {
                self.editing_details = false;
                match key_event.code {
                    KeyCode::Char(to_insert) => {
                        self.search_input
                            .insert(self.search_character_index, to_insert);
                        self.search_character_index += 1;
                    }
                    KeyCode::Backspace => {
                        if self.search_character_index > 0 {
                            self.search_input
                                .remove(self.search_character_index.saturating_sub(1));
                            self.search_character_index =
                                self.search_character_index.saturating_sub(1);
                        }
                    }
                    KeyCode::Left => {
                        self.search_character_index = self.search_character_index.saturating_sub(1);
                    }
                    KeyCode::Right => {
                        self.search_character_index =
                            min(self.search_character_index + 1, self.search_input.len())
                    }
                    KeyCode::Enter => {
                        self.search_character_index = 0;
                        self.search_input.clear();
                    }
                    _ => {}
                };
                self.cursor_position.y = 1;
                self.cursor_position.x = (self.search_character_index + 1) as u16;
            }
            DatabaseFrame::Assets => {
                self.details_scroll = 0;
                match key_event.code {
                    KeyCode::Up => {
                        self.details_index = 0;
                        self.visible_assets.previous()
                    }

                    KeyCode::Down => {
                        self.details_index = 0;
                        self.visible_assets.next()
                    }
                    _ => {}
                };
            }
            DatabaseFrame::Details => {
                if self.editing_details {
                    match key_event.code {
                        KeyCode::Char(to_insert) => {
                            self.details_input
                                .insert(self.details_character_index, to_insert);
                            self.details_character_index += 1;
                        }
                        KeyCode::Backspace => {
                            if self.details_character_index > 0 {
                                self.details_input
                                    .remove(self.details_character_index.saturating_sub(1));
                                self.details_character_index =
                                    self.details_character_index.saturating_sub(1);
                            }
                        }
                        KeyCode::Left => {
                            self.details_character_index =
                                self.details_character_index.saturating_sub(1);
                        }
                        KeyCode::Right => {
                            self.details_character_index =
                                min(self.details_character_index + 1, self.details_input.len());
                        }
                        KeyCode::Enter => {
                            match self.current_asset.asset_type {
                                AssetType::Aura => {
                                    let mut aura = self.aura_lib.id(self.current_asset.id).clone();
                                    set_field_value_from_string(
                                        &mut aura,
                                        &self.current_asset_fields[self.details_index],
                                        self.details_input.clone(),
                                    );
                                    self.aura_lib.update_def(aura.into());
                                }
                                AssetType::Item => {
                                    let mut item = self.item_lib.id(self.current_asset.id).clone();
                                    set_field_value_from_string(
                                        &mut item,
                                        &self.current_asset_fields[self.details_index],
                                        self.details_input.clone(),
                                    );
                                    self.item_lib.update_def(item.into());
                                }
                            };
                            self.details_character_index = 0;
                            self.details_input.clear();
                            self.editing_details = false;
                        }
                        _ => {}
                    };
                } else {
                    match key_event.code {
                        KeyCode::Up => match key_event.modifiers {
                            KeyModifiers::SHIFT => {
                                self.details_index = self.details_index.saturating_sub(1);
                            }
                            _ => {
                                self.details_scroll = self.details_scroll.saturating_sub(1);
                            }
                        },
                        KeyCode::Down => match key_event.modifiers {
                            KeyModifiers::SHIFT => {
                                self.details_index = min(
                                    self.details_index.saturating_add(1),
                                    self.current_asset_fields.len() - 1,
                                );
                            }
                            _ => {
                                self.details_scroll = min(
                                    self.details_scroll.saturating_add(1),
                                    self.max_details_scroll,
                                );
                            }
                        },
                        KeyCode::Enter => {
                            self.editing_details = true;
                            self.details_input = match self.current_asset.asset_type {
                                AssetType::Aura => get_string_value_from_path(
                                    self.aura_lib.id(self.current_asset.id),
                                    &self.current_asset_fields[self.details_index],
                                ),
                                AssetType::Item => get_string_value_from_path(
                                    self.item_lib.id(self.current_asset.id),
                                    &self.current_asset_fields[self.details_index],
                                ),
                            };
                            self.details_character_index = self.details_input.len();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

impl Widget for &mut Database {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal_sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Length(area.height - 3),
            ])
            .split(area);
        let vertical_sections = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(20),
                Constraint::Length(area.width - 20),
            ])
            .split(horizontal_sections[1]);
        self.render_search_bar(horizontal_sections[0], buf);
        self.render_assets(vertical_sections[0], buf);
        self.get_cursor_position(vertical_sections[1], buf);
        self.render_details(vertical_sections[1], buf, false);
    }
}

impl Database {
    fn get_title_style(&self, frame: DatabaseFrame) -> Style {
        if self.active_frame == frame {
            Style::default().fg(self.window.theme.green)
        } else {
            Style::default().fg(self.window.theme.white)
        }
    }

    fn render_assets(&mut self, area: Rect, buf: &mut Buffer) {
        // TODO: Add Markers for asset types. ◆ ■ ○ ●
        self.populate_visible_assets();
        let mut list_items = vec![];
        for index in &self.visible_assets.assets {
            list_items.push(self.assets[*index].to_list_item())
        }

        let asset_list = List::new(list_items)
            .block(
                Block::default()
                    .title("Assets")
                    .borders(Borders::ALL)
                    .border_type(self.window.border_type)
                    .style(Style::default().fg(self.window.theme.white))
                    .title_style(self.get_title_style(DatabaseFrame::Assets)),
            )
            .bg(self.window.theme.black_dark)
            .fg(self.window.theme.white)
            .highlight_style(Style::default().fg(self.window.theme.red))
            .direction(ListDirection::TopToBottom);

        StatefulWidget::render(&asset_list, area, buf, &mut self.visible_assets.state);

        if asset_list.len() > 0 {
            let scroll = asset_list.len().saturating_sub((area.height - 1) as usize);
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .track_symbol(Some(self.window.border_type.to_border_set().vertical_left))
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"));

            let mut scrollbar_state =
                ScrollbarState::new(scroll).position(self.visible_assets.state.selected().unwrap());

            StatefulWidget::render(
                scrollbar,
                area.inner(Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                buf,
                &mut scrollbar_state,
            );
        }
    }

    fn render_details(&mut self, area: Rect, buf: &mut Buffer, render_cursor: bool) {
        if self.visible_assets.assets.is_empty() {
            self.render_empty_details(area, buf);
            return;
        }
        let asset = self.assets
            [self.visible_assets.assets[self.visible_assets.state.selected().unwrap()]]
        .clone();
        let img = self.get_icon(&asset);
        let icon_width = min(area.width / 4, img.width() as u16);
        let sections = self.build_details_sections(area, icon_width);

        self.current_asset = asset.clone();

        let full_details = match asset.asset_type {
            AssetType::Aura => {
                self.current_asset_fields = get_def_paths(self.aura_lib.id(asset.id));
                self.add_aura_details(&asset, render_cursor)
            }
            AssetType::Item => {
                self.current_asset_fields = get_def_paths(self.item_lib.id(asset.id));
                self.add_item_details(&asset, render_cursor)
            }
        };

        let p = self.build_details_paragraph(full_details);

        let max_details_scroll =
            (p.line_count(sections[0].width) as u16).saturating_sub(area.height - 1);
        p.clone()
            .scroll((min(self.details_scroll, max_details_scroll), 0))
            .render(sections[0], buf);

        self.max_details_scroll = max_details_scroll;

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .track_symbol(Some(self.window.border_type.to_border_set().vertical_left))
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));

        let mut scrollbar_state =
            ScrollbarState::new(max_details_scroll.into()).position(self.details_scroll.into());

        StatefulWidget::render(
            scrollbar,
            sections[0].inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            buf,
            &mut scrollbar_state,
        );

        terminal_image::render_image_with_border(
            sections[1],
            buf,
            img,
            self.window.theme,
            self.window.border_type,
        );
    }

    fn build_details_sections(&mut self, area: Rect, icon_width: u16) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(area.width - icon_width),
                Constraint::Length(icon_width),
            ])
            .split(area)
    }

    fn build_details_paragraph<'a>(&self, contents: Vec<Line<'a>>) -> Paragraph<'a> {
        Paragraph::new(contents)
            .block(
                Block::default()
                    .title("Details")
                    .borders(Borders::ALL)
                    .border_type(self.window.border_type)
                    .style(Style::default().fg(self.window.theme.white))
                    .title_style(self.get_title_style(DatabaseFrame::Details)),
            )
            .bg(self.window.theme.black_dark)
            .fg(self.window.theme.white)
            .wrap(Wrap { trim: true })
    }

    fn render_empty_details(&mut self, area: Rect, buf: &mut Buffer) {
        let sections = self.build_details_sections(area, min(area.width / 5, 32));
        let p = self.build_details_paragraph(vec![]);
        p.render(sections[0], buf);
        terminal_image::render_image_path_with_border(
            sections[1],
            buf,
            UNKNOWN_IMAGE_PATH,
            self.window.theme,
            self.window.border_type,
        );
    }

    fn get_icon(&self, asset: &Asset) -> DynamicImage {
        let path = match asset.asset_type {
            AssetType::Aura => self.aura_lib.id(asset.id).icon.clone(),
            AssetType::Item => self.item_lib.id(asset.id).icon.clone(),
        };
        load_image(&format!("asset/{}", &path))
    }

    fn add_aura_details(&self, asset: &Asset, with_cursor_marker: bool) -> Vec<Line> {
        let mut details = vec![];
        let aura = self.aura_lib.id(asset.id);
        for path in &self.current_asset_fields {
            details.push(self.format_detail(
                path,
                &get_string_value_from_path(aura, path),
                with_cursor_marker,
            ));
        }
        details
    }

    fn add_item_details(&self, asset: &Asset, with_cursor_marker: bool) -> Vec<Line> {
        let mut details = vec![];
        let item = self.item_lib.id(asset.id);
        for path in &self.current_asset_fields {
            details.push(self.format_detail(
                path,
                &get_string_value_from_path(item, path),
                with_cursor_marker,
            ));
        }
        details
    }

    fn format_detail<T: Display + Reflect>(
        &self,
        path: &str,
        contents: &T,
        with_cursor_marker: bool,
    ) -> Line {
        // This is a bit hacky and probably has issues. If it parses to an f64
        // then assume its a number and color it red.
        let contents_color = if contents.to_string().parse::<f64>().is_ok() {
            self.window.theme.red
        } else {
            self.window.theme.white
        };

        let mut contents = contents.to_string();
        let field_color = if self.current_asset_fields[self.details_index] == path {
            if self.editing_details {
                contents = self.details_input.clone();
                if with_cursor_marker {
                    contents.replace_range(
                        self.details_character_index.saturating_sub(1)
                            ..self.details_character_index,
                        MAGIC_CURSOR_SYMBOL,
                    );
                }
                self.window.theme.green
            } else {
                self.window.theme.red
            }
        } else {
            self.window.theme.blue
        };

        vec![
            Span::styled(format!("{}: ", path), Style::default().fg(field_color)),
            Span::styled(contents.to_string(), Style::default().fg(contents_color)),
        ]
        .into()
    }

    fn render_search_bar(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.search_input.to_string())
            .block(
                Block::default()
                    .title("Search")
                    .borders(Borders::ALL)
                    .border_type(self.window.border_type)
                    .style(Style::default().fg(self.window.theme.white))
                    .title_style(self.get_title_style(DatabaseFrame::Search)),
            )
            .bg(self.window.theme.black_dark)
            .fg(self.window.theme.white)
            .render(area, buf);
    }

    fn populate_visible_assets(&mut self) {
        self.visible_assets.assets = vec![];
        for i in 0..self.assets.len() {
            // TODO: Implement a more sophisticated search
            if self.assets[i]
                .name
                .to_lowercase()
                .starts_with(&self.search_input.to_lowercase())
            {
                self.visible_assets.assets.push(i)
            }
        }
        self.visible_assets.clamp();
    }

    // This is a horrible, terrible hack. We render the entire details frame twice
    // to a duplicate buffer with a special character, and then iterate through
    // the entire frame until we find the absolute position of that special
    // character, and use that as our cursor location.
    //
    // TODO: Fix incorrect cursor behaviour that occurs when deleting characters
    // causes the text to unwrap.
    // TODO: Get rid of this entirely once Ratatui has better support for getting
    // cursor position from wrapped text.
    fn get_cursor_position(&mut self, area: Rect, buf: &mut Buffer) {
        let mut fake_buf = buf.clone();
        self.render_details(area, &mut fake_buf, true);
        for x in 0..area.width {
            for y in 0..area.height {
                if let Some(character) = fake_buf.cell(Position { x, y }) {
                    if character.symbol() == MAGIC_CURSOR_SYMBOL {
                        if self.details_character_index == 0 {
                            self.cursor_position = Position { x, y };
                        } else {
                            self.cursor_position = Position { x: x + 1, y };
                        }

                        break;
                    }
                }
            }
        }
    }
}

fn get_def_paths_helper(def: &dyn Struct, current_path: &str, paths: &mut Vec<String>) {
    for (i, field) in def.iter_fields().enumerate() {
        match field.reflect_ref().as_struct() {
            Err(_) => {
                if current_path.is_empty() {
                    paths.push(def.name_at(i).unwrap().to_string());
                } else {
                    paths.push(format!("{}.{}", current_path, def.name_at(i).unwrap()));
                }
            }
            Ok(sub_field) => get_def_paths_helper(sub_field, def.name_at(i).unwrap(), paths),
        }
    }
}

fn get_def_paths(def: &dyn Struct) -> Vec<String> {
    let mut field_paths: Vec<String> = vec![];
    get_def_paths_helper(def, "", &mut field_paths);
    field_paths
}

fn get_string_value_from_path<T: PartialReflect + GetPath>(def: &T, path: &str) -> String {
    // Numeric Types
    if let Ok(value) = def.path::<u32>(path) {
        return (*value).to_string();
    } else if let Ok(value) = def.path::<u64>(path) {
        return (*value).to_string();
    } else if let Ok(value) = def.path::<i32>(path) {
        return (*value).to_string();
    } else if let Ok(value) = def.path::<i64>(path) {
        return (*value).to_string();
    } else if let Ok(value) = def.path::<f32>(path) {
        return (*value).to_string();
    } else if let Ok(value) = def.path::<f64>(path) {
        return (*value).to_string();
    } else if let Ok(value) = def.path::<f64>(path) {
        return (*value).to_string();

    // String
    } else if let Ok(value) = def.path::<String>(path) {
        return (*value).to_string();
    } else if let Ok(value) = def.path::<&str>(path) {
        return (*value).to_string();

    // Enums
    // TODO: Find some way to handle enums more generally.
    } else if let Ok(value) = def.path::<ItemType>(path) {
        return (*value).to_string();
    } else if let Ok(value) = def.path::<ItemRarity>(path) {
        return (*value).to_string();
    } else if let Ok(value) = def.path::<EquipmentSlot>(path) {
        return (*value).to_string();
    } else if let Ok(value) = def.path::<AuraType>(path) {
        return (*value).to_string();
    }

    "UNKNOWN_TYPE".to_string()
}

fn set_field_value_from_string<T: PartialReflect + GetPath>(
    def: &mut T,
    path: &str,
    new_value: String,
) {
    // TODO: Don't panic when failing to parse. Come up with a way
    // to indicate to the user that a failure occurred.
    // Numeric Types
    if let Ok(value) = def.path_mut::<u32>(path) {
        *value = new_value.parse::<u32>().unwrap();
    } else if let Ok(value) = def.path_mut::<u64>(path) {
        *value = new_value.parse::<u64>().unwrap();
    } else if let Ok(value) = def.path_mut::<i32>(path) {
        *value = new_value.parse::<i32>().unwrap();
    } else if let Ok(value) = def.path_mut::<i64>(path) {
        *value = new_value.parse::<i64>().unwrap();
    } else if let Ok(value) = def.path_mut::<f32>(path) {
        *value = new_value.parse::<f32>().unwrap();
    } else if let Ok(value) = def.path_mut::<f64>(path) {
        *value = new_value.parse::<f64>().unwrap();

    // String
    } else if let Ok(value) = def.path_mut::<String>(path) {
        *value = new_value

    // Enums
    } else if let Ok(value) = def.path_mut::<ItemType>(path) {
        *value = new_value.parse::<ItemType>().unwrap();
    } else if let Ok(value) = def.path_mut::<ItemRarity>(path) {
        *value = new_value.parse::<ItemRarity>().unwrap();
    } else if let Ok(value) = def.path_mut::<EquipmentSlot>(path) {
        *value = new_value.parse::<EquipmentSlot>().unwrap();
    } else if let Ok(value) = def.path_mut::<AuraType>(path) {
        *value = new_value.parse::<AuraType>().unwrap();
    }
}

impl AssetList {
    fn from_assets(assets: Vec<usize>) -> AssetList {
        AssetList {
            state: ListState::default(),
            assets,
        }
    }

    fn next(&mut self) {
        if self.state.selected().unwrap() < self.assets.len() - 1 {
            self.state.select(Some(self.state.selected().unwrap() + 1));
        }
    }

    fn previous(&mut self) {
        if self.state.selected().unwrap() > 0 {
            self.state.select(Some(self.state.selected().unwrap() - 1));
        }
    }

    fn clamp(&mut self) {
        if self.state.selected().is_some() {
            if self.state.selected().unwrap() >= self.assets.len() {
                self.state.select(Some(self.assets.len().saturating_sub(1)));
            }
        } else {
            self.state.select(Some(0));
        }
    }
}

impl Asset {
    fn to_list_item(&self) -> ListItem {
        ListItem::new(self.name.to_string())
    }
}
