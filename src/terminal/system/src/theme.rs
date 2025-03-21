use ratatui::style::Color;

#[derive(Debug, Copy, Clone)]
pub struct Theme {
    pub white_light: Color,
    pub black_light: Color,
    pub red_light: Color,
    pub green_light: Color,
    pub blue_light: Color,
    pub cyan_light: Color,
    pub yellow_light: Color,
    pub magenta_light: Color,

    pub white: Color,
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub blue: Color,
    pub cyan: Color,
    pub yellow: Color,
    pub magenta: Color,

    pub white_dark: Color,
    pub black_dark: Color,
    pub red_dark: Color,
    pub green_dark: Color,
    pub blue_dark: Color,
    pub cyan_dark: Color,
    pub yellow_dark: Color,
    pub magenta_dark: Color,
}

impl Theme {
    pub const AMBER: Theme = Theme {
        white_light: Color::from_u32(0xfca21b),
        black_light: Color::from_u32(0x282423),
        red_light: Color::from_u32(0xf75b40),
        green_light: Color::from_u32(0xB5B21B),
        blue_light: Color::from_u32(0x6082A3),
        cyan_light: Color::from_u32(0x68c4c2),
        yellow_light: Color::from_u32(0xefc807),
        magenta_light: Color::from_u32(0x7155d6),

        white: Color::from_u32(0xF8881D),
        black: Color::from_u32(0x1B1817),
        red: Color::from_u32(0xDE4227),
        green: Color::from_u32(0x949216),
        blue: Color::from_u32(0x4F6A83),
        cyan: Color::from_u32(0x5d9996),
        yellow: Color::from_u32(0xba9c0b),
        magenta: Color::from_u32(0x544689),

        white_dark: Color::from_u32(0xa03500),
        black_dark: Color::from_u32(0x130E0E),
        red_dark: Color::from_u32(0xA71800),
        green_dark: Color::from_u32(0x5a5b01),
        blue_dark: Color::from_u32(0x2c3e4f),
        cyan_dark: Color::from_u32(0x3e666b),
        yellow_dark: Color::from_u32(0xa57f0d),
        magenta_dark: Color::from_u32(0x372e56),
    };

    // Linearly interpolates between two different colors
    pub fn lerp(c1: Color, c2: Color, w: f32) -> Color {
        let (r1, g1, b1) = match c1 {
            Color::Rgb(r, g, b) => (r, g, b),
            _ => (0, 0, 0),
        };

        let (r2, g2, b2) = match c2 {
            Color::Rgb(r, g, b) => (r, g, b),
            _ => (0, 0, 0),
        };

        let r = if r1 > r2 {
            r2 + (((r1 - r2) as f32) * w) as u8
        } else {
            r1 + (((r2 - r1) as f32) * w) as u8
        };

        let g = if g1 > g2 {
            g2 + (((g1 - g2) as f32) * w) as u8
        } else {
            g1 + (((g2 - g1) as f32) * w) as u8
        };

        let b = if b1 > b2 {
            b2 + (((b1 - b2) as f32) * w) as u8
        } else {
            b1 + (((b2 - b1) as f32) * w) as u8
        };

        Color::Rgb(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        let c1 = Color::Rgb(200, 100, 50);
        let c2 = Color::Rgb(0, 0, 0);
        assert_eq!(Color::Rgb(100, 50, 25), Theme::lerp(c1, c2, 0.5));
        assert_eq!(Color::Rgb(100, 50, 25), Theme::lerp(c2, c1, 0.5));

        let c1 = Color::Rgb(200, 100, 50);
        let c2 = Color::Rgb(0, 0, 0);
        assert_eq!(c1, Theme::lerp(c1, c2, 1.0));
        assert_eq!(c2, Theme::lerp(c1, c2, 0.0));

        let c1 = Color::Rgb(3, 5, 7);
        let c2 = Color::Rgb(0, 0, 0);
        assert_eq!(Color::Rgb(1, 2, 3), Theme::lerp(c1, c2, 0.5));
    }
}
