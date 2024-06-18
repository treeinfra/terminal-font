use crate::{cancel, custom::*, decorate::Decorate};

#[derive(Clone, Copy)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGBColor {
    pub fn from(r: u8, g: u8, b: u8) -> RGBColor {
        RGBColor { r, g, b }
    }
}

/// Encapsulation on the [Decorate] trait with
/// custom color escape codes for style decorations,
/// including RGB mode ([RGBColor]) and color code ([u8]) mode.
///
/// Those methods won't check whether there are conflicts in the raw string.
/// The only thing to do is wrap the prefix and the suffix.
/// Those methods are usually used when content is predicated,
/// and performance is more important.
pub trait SimpleCustomColor<T>: Decorate {
    fn simple_fg(&self, code: T) -> String;
    fn simple_bg(&self, code: T) -> String;
}

/// Color code ([u8]) version of custom color decoration.
/// ```rust
/// use terminal_font::custom_color::*;
/// assert_eq!(" hello ".simple_fg(123), "\x1b[38;5;123m hello \x1b[39m");
/// assert_eq!(" hello ".simple_bg(123), "\x1b[48;5;123m hello \x1b[49m");
/// ```
impl<T: Decorate + AsRef<str>> SimpleCustomColor<u8> for T {
    fn simple_fg(&self, code: u8) -> String {
        self.wrap(foreground_code(code), cancel::FOREGROUND)
    }

    fn simple_bg(&self, code: u8) -> String {
        self.wrap(background_code(code), cancel::BACKGROUND)
    }
}

/// RGB ([RGBColor]) version of custom color decoration.
/// ```rust
/// use terminal_font::custom_color::*;
/// let c = RGBColor::from(143, 76, 78);
/// assert_eq!(" hello ".simple_fg(c), "\x1b[38;2;143;76;78m hello \x1b[39m");
/// assert_eq!(" hello ".simple_bg(c), "\x1b[48;2;143;76;78m hello \x1b[49m");
/// ```
impl<T: Decorate + AsRef<str>> SimpleCustomColor<RGBColor> for T {
    fn simple_fg(&self, color: RGBColor) -> String {
        self.wrap(
            foreground_rgb(color.r, color.g, color.b),
            cancel::FOREGROUND,
        )
    }

    fn simple_bg(&self, color: RGBColor) -> String {
        self.wrap(
            background_rgb(color.r, color.g, color.b),
            cancel::BACKGROUND,
        )
    }
}
