use crate::theme::Theme;
use image::{imageops::FilterType::Nearest, DynamicImage, GenericImageView};

use ratatui::{prelude::*, widgets::BorderType};

pub const UNKNOWN_IMAGE_PATH: &str = "asset/sprite/icon/unknown.png";

// Renders an image to the screen. Two pixels exist in a single space, where the ▀
// character represents the top pixel and the background color represents the bottom pixel
fn render(pos_x: u16, pos_y: u16, buf: &mut Buffer, img: DynamicImage, theme: Theme) {
    for x in 0..img.width() {
        for y in (0..img.height() - 1).step_by(2) {
            let fg_p = img.get_pixel(x, y);
            let fg_color = Color::Rgb(fg_p[0], fg_p[1], fg_p[2]);
            let mut bg_color = theme.black_dark;
            if y < img.width() {
                let bg_p = img.get_pixel(x, y + 1);
                bg_color = Color::Rgb(bg_p[0], bg_p[1], bg_p[2]);
            }
            buf.cell_mut(Position {
                x: (x as u16) + pos_x,
                y: ((y / 2) as u16) + pos_y,
            })
            .unwrap()
            .set_char('▀')
            .set_fg(fg_color)
            .set_bg(bg_color);
        }
    }
}

// Renders an image and draws a border around it.
pub fn render_image_with_border(
    area: Rect,
    buf: &mut Buffer,
    img: DynamicImage,
    theme: Theme,
    border: BorderType,
) {
    let pos_x = area.left();
    let pos_y = area.top();
    // The area inside the border
    let image_area = Rect {
        x: pos_x + 1,
        y: pos_y + 1,
        width: area.width - 2,
        height: area.height - 2,
    };
    let img = resize_image(image_area, img);

    let end_x = pos_x + (img.width() as u16) + 1;
    let end_y = pos_y + ((img.height() / 2) as u16) + 1;
    render(pos_x + 1, pos_y + 1, buf, img.clone(), theme);

    let bs = border.to_border_set();

    // Get the individual ascii characters that constitute the border
    let horizontal_border = bs.horizontal_top.chars().next().unwrap();
    let vertical_border = bs.vertical_left.chars().next().unwrap();
    let top_left_border = bs.top_left.chars().next().unwrap();
    let top_right_border = bs.top_right.chars().next().unwrap();
    let bottom_right_border = bs.bottom_right.chars().next().unwrap();
    let bottom_left_border = bs.bottom_left.chars().next().unwrap();

    // Draw the top border
    for x in pos_x..end_x {
        buf.cell_mut(Position { x: x, y: pos_y })
            .unwrap()
            .set_char(horizontal_border)
            .set_fg(theme.white)
            .set_bg(theme.black_dark);
    }

    // Draw the vertical borders
    for y in pos_y..end_y {
        buf.cell_mut(Position { x: pos_x, y: y })
            .unwrap()
            .set_char(vertical_border)
            .set_fg(theme.white)
            .set_bg(theme.black_dark);
        buf.cell_mut(Position { x: end_x, y: y })
            .unwrap()
            .set_char(vertical_border)
            .set_fg(theme.white)
            .set_bg(theme.black_dark);
    }

    // Draw the bottom border
    for x in pos_x..end_x {
        buf.cell_mut(Position { x: x, y: end_y })
            .unwrap()
            .set_char(horizontal_border)
            .set_fg(theme.white)
            .set_bg(theme.black_dark);
    }

    // Draw the corners
    buf.cell_mut(Position { x: pos_x, y: pos_y })
        .unwrap()
        .set_char(top_left_border)
        .set_fg(theme.white)
        .set_bg(theme.black_dark);

    buf.cell_mut(Position { x: end_x, y: pos_y })
        .unwrap()
        .set_char(top_right_border)
        .set_fg(theme.white)
        .set_bg(theme.black_dark);

    buf.cell_mut(Position { x: pos_x, y: end_y })
        .unwrap()
        .set_char(bottom_left_border)
        .set_fg(theme.white)
        .set_bg(theme.black_dark);

    buf.cell_mut(Position { x: end_x, y: end_y })
        .unwrap()
        .set_char(bottom_right_border)
        .set_fg(theme.white)
        .set_bg(theme.black_dark);
}

fn resize_image(area: Rect, img: DynamicImage) -> DynamicImage {
    let area_width = area.width as u32;
    let area_height = (area.height * 2) as u32;

    // Since each space is two pixels tall, we mutliply height by 2
    // to avoid stretching the image.
    if img.width() > area_width && img.height() > area_height {
        return img.resize(area_width, area_height, Nearest);
    }
    if img.width() > area_width {
        return img.resize(area_width, img.height(), Nearest);
    }
    if img.height() > area_height {
        return img.resize(img.width(), area_height, Nearest);
    }
    img
}

pub fn render_image(area: Rect, buf: &mut Buffer, img: DynamicImage, theme: Theme) {
    let img = resize_image(area, img);
    render(area.left(), area.top(), buf, img, theme);
}

pub fn render_image_path(area: Rect, buf: &mut Buffer, image_path: &str, theme: Theme) {
    let mut img = image::open(image_path).unwrap_or(image::open(UNKNOWN_IMAGE_PATH).unwrap());
    img = resize_image(area, img);
    render(area.left(), area.top(), buf, img, theme);
}

pub fn render_image_path_with_border(
    area: Rect,
    buf: &mut Buffer,
    image_path: &str,
    theme: Theme,
    border: BorderType,
) {
    let img = load_image(image_path);
    render_image_with_border(area, buf, img, theme, border);
}

pub fn load_image(image_path: &str) -> DynamicImage {
    image::open(image_path).unwrap_or(image::open(UNKNOWN_IMAGE_PATH).unwrap())
}

// Overwrites the background color of an entire area to the `color`
pub fn set_background_color(area: Rect, buf: &mut Buffer, color: Color) {
    for x in 0..area.width {
        for y in 0..area.height {
            buf.cell_mut(Position { x, y }).unwrap().set_bg(color);
        }
    }
}
