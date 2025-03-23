use term_screen::database::Database;
use term_screen::menu::Menu;
use term_system::tui;
use term_system::window::{Screen, Window, WindowName};

use std::io;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let mut current_window = WindowName::Menu;
    let mut menu = Menu::new(Window::default());
    let mut database = Database::new(Window::default());
    while current_window != WindowName::None {
        let window_result = match current_window {
            WindowName::Menu => menu.run(&mut terminal),
            WindowName::Database => database.run(&mut terminal),
            _ => Ok(WindowName::None),
        };
        current_window = match window_result {
            Err(ref error) => {
                println!("Encountered an error: {:?}", error);
                WindowName::None
            }
            Ok(window) => window,
        };
    }
    tui::restore()?;

    Ok(())
}

#[cfg(test)]
mod tests {}
