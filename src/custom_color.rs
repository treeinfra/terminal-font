use crate::{cancel, custom::*, decorate::Decorate};

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
/// including RGB mode ([RGBColor]) and color code mode.
///
/// Those methods won't check whether there are conflicts in the raw string.
/// The only thing to do is wrap the prefix and the suffix.
/// Those methods are usually used when content is predicated,
/// and performance is more important.
///
/// ```rust
/// use terminal_font::custom_color::*;
///
/// assert_eq!(
///     " hello ".simple_rgb(RGBColor::from(143, 76, 78)),
///     "\x1b[38;2;143;76;78m hello \x1b[39m",
/// );
///
/// assert_eq!(
///     " hello ".simple_bg_rgb(RGBColor::from(143, 76, 78)),
///     "\x1b[48;2;143;76;78m hello \x1b[49m",
/// );
///
/// assert_eq!(" hello ".simple_fg(123), "\x1b[38;5;123m hello \x1b[39m");
/// assert_eq!(" hello ".simple_bg(123), "\x1b[48;5;123m hello \x1b[49m");
/// ```
pub trait SimpleCustomColor: Decorate {
    fn simple_rgb(&self, color: RGBColor) -> String {
        self.wrap(
            foreground_rgb(color.r, color.g, color.b),
            cancel::FOREGROUND,
        )
    }

    fn simple_bg_rgb(&self, color: RGBColor) -> String {
        self.wrap(
            background_rgb(color.r, color.g, color.b),
            cancel::BACKGROUND,
        )
    }

    fn simple_fg(&self, code: u8) -> String {
        self.wrap(foreground_code(code), cancel::FOREGROUND)
    }

    fn simple_bg(&self, code: u8) -> String {
        self.wrap(background_code(code), cancel::BACKGROUND)
    }
}

impl<T: Decorate + AsRef<str>> SimpleCustomColor for T {}
