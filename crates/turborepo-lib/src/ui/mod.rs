use std::f64::consts::PI;

use console::{Style, StyledObject};
use lazy_static::lazy_static;

/// Helper struct to apply any necessary formatting to UI output
pub struct UI {
    should_strip_ansi: bool,
}

impl UI {
    pub fn new(should_strip_ansi: bool) -> Self {
        Self { should_strip_ansi }
    }

    /// Infer the color choice from environment variables and checking if stdout
    /// is a tty
    pub fn infer() -> Self {
        let env_setting =
            std::env::var("FORCE_COLOR")
                .ok()
                .and_then(|force_color| match force_color.as_str() {
                    "false" | "0" => Some(true),
                    "true" | "1" | "2" | "3" => Some(false),
                    _ => None,
                });
        let should_strip_ansi = env_setting.unwrap_or_else(|| !atty::is(atty::Stream::Stdout));
        Self { should_strip_ansi }
    }

    /// Apply the UI color mode to the given styled object
    ///
    /// This is required to match the Go turborepo coloring logic which differs
    /// from console's coloring detection.
    pub fn apply<D>(&self, obj: StyledObject<D>) -> StyledObject<D> {
        // Setting this to false will skip emitting any ansi codes associated
        // with the style when the object is displayed.
        obj.force_styling(!self.should_strip_ansi)
    }

    // Ported from Go code. Converts an index to a color along the rainbow
    fn rainbow_rgb(i: usize) -> (u8, u8, u8) {
        let f = 0.275;
        let r = (f * i as f64 + 4.0 * PI / 3.0).sin() * 127.0 + 128.0;
        let g = 45.0;
        let b = (f * i as f64).sin() * 127.0 + 128.0;

        (r as u8, g as u8, b as u8)
    }

    pub fn print_rainbow(&self, text: &str) {
        if self.should_strip_ansi {
            println!("{}", text);
            return;
        }
        for (i, c) in text.char_indices() {
            let (r, g, b) = Self::rainbow_rgb(i);
            print!("\x1b[1m\x1b[38;2;{};{};{}m{}\x1b[0m\x1b[0;1m", r, g, b, c);
        }
        println!()
    }
}

lazy_static! {
    pub static ref GREY: Style = Style::new().dim();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rainbow() {
        let ui = UI::infer();
        ui.print_rainbow("hello world");
    }

    #[test]
    fn test_ui_strips_ansi() {
        let ui = UI::new(true);
        let grey_str = GREY.apply_to("gray");
        assert_eq!(format!("{}", ui.apply(grey_str)), "gray");
    }

    #[test]
    fn test_ui_resets_term() {
        let ui = UI::new(false);
        let grey_str = GREY.apply_to("gray");
        assert_eq!(format!("{}", ui.apply(grey_str)), "\u{1b}[2mgray\u{1b}[0m");
    }
}
