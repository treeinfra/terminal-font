use crate::escape::style::*;
use crate::escape::{cancel, foreground};

pub trait Decorate {
    /// Simply wrap a prefix and a suffix.
    ///
    /// This method is seldom directly used.
    /// You can find more encapsulations with escape codes
    /// in the [SimpleStyle], [SimpleForeground]
    /// and [SimpleBackground] supertraits.
    ///
    /// ```rust
    /// use terminal_font::decorate::Decorate;
    /// assert_eq!("hello".wrap("[", "]"), "[hello]");
    /// ```
    fn wrap(&self, prefix: &str, suffix: &str) -> String;
}

impl<T: AsRef<str>> Decorate for T {
    fn wrap(&self, prefix: &str, suffix: &str) -> String {
        format!("{}{}{}", prefix, self.as_ref(), suffix)
    }
}

/// Encapsulation on the [Decorate] trait with
/// commonly used escape codes for style decorations.
///
/// Those methods won't check whether there are conflicts in the raw string.
/// The only thing to do is wrap the prefix and the suffix.
/// Those methods are usually used when content is predicated,
/// and performance is more important.
///
/// ```rust
/// use terminal_font::decorate::SimpleStyle;
/// assert_eq!(" hello ".simple_bold(), "\x1b[1m hello \x1b[22m");
/// assert_eq!(" hello ".simple_faint(), "\x1b[2m hello \x1b[22m");
/// assert_eq!(" hello ".simple_italic(), "\x1b[3m hello \x1b[23m");
/// assert_eq!(" hello ".simple_underline(), "\x1b[4m hello \x1b[24m");
/// assert_eq!(" hello ".simple_blink(), "\x1b[5m hello \x1b[25m");
/// assert_eq!(" hello ".simple_blink_fast(), "\x1b[6m hello \x1b[25m");
/// assert_eq!(" hello ".simple_inverse(), "\x1b[7m hello \x1b[27m");
/// assert_eq!(" hello ".simple_conceal(), "\x1b[8m hello \x1b[28m");
/// assert_eq!(" hello ".simple_strikethrough(), "\x1b[9m hello \x1b[29m");
/// assert_eq!(" hello ".simple_double_underline(), "\x1b[21m hello \x1b[24m");
/// ```
pub trait SimpleStyle: Decorate {
    fn simple_bold(&self) -> String {
        self.wrap(BOLD, cancel::BOLD_OR_FAINT)
    }

    fn simple_faint(&self) -> String {
        self.wrap(FAINT, cancel::BOLD_OR_FAINT)
    }

    fn simple_italic(&self) -> String {
        self.wrap(ITALIC, cancel::ITALIC)
    }

    fn simple_underline(&self) -> String {
        self.wrap(UNDERLINE, cancel::UNDERLINE)
    }

    fn simple_blink(&self) -> String {
        self.wrap(BLINK, cancel::BLINK)
    }

    fn simple_blink_fast(&self) -> String {
        self.wrap(BLINK_FAST, cancel::BLINK)
    }

    fn simple_inverse(&self) -> String {
        self.wrap(INVERSE, cancel::INVERSE)
    }

    fn simple_conceal(&self) -> String {
        self.wrap(CONCEAL, cancel::CONCEAL)
    }

    fn simple_strikethrough(&self) -> String {
        self.wrap(STRIKETHROUGH, cancel::STRIKETHROUGH)
    }

    fn simple_double_underline(&self) -> String {
        self.wrap(DOUBLE_UNDERLINE, cancel::UNDERLINE)
    }
}

/// Encapsulation on the [Decorate] trait with
/// commonly used escape codes for style decorations.
///
/// Those methods won't check whether there are conflicts in the raw string.
/// The only thing to do is wrap the prefix and the suffix.
/// Those methods are usually used when content is predicated,
/// and performance is more important.
///
/// ```rust
/// use terminal_font::decorate::SimpleForeground;
/// assert_eq!(" hello ".simple_black(), "\x1b[30m hello \x1b[39m");
/// assert_eq!(" hello ".simple_red(), "\x1b[31m hello \x1b[39m");
/// assert_eq!(" hello ".simple_green(), "\x1b[32m hello \x1b[39m");
/// assert_eq!(" hello ".simple_yellow(), "\x1b[33m hello \x1b[39m");
/// assert_eq!(" hello ".simple_blue(), "\x1b[34m hello \x1b[39m");
/// assert_eq!(" hello ".simple_magenta(), "\x1b[35m hello \x1b[39m");
/// assert_eq!(" hello ".simple_cyan(), "\x1b[36m hello \x1b[39m");
/// assert_eq!(" hello ".simple_white(), "\x1b[37m hello \x1b[39m");
/// assert_eq!(" hello ".simple_hi_black(), "\x1b[90m hello \x1b[39m");
/// assert_eq!(" hello ".simple_hi_red(), "\x1b[91m hello \x1b[39m");
/// assert_eq!(" hello ".simple_hi_green(), "\x1b[92m hello \x1b[39m");
/// assert_eq!(" hello ".simple_hi_yellow(), "\x1b[93m hello \x1b[39m");
/// assert_eq!(" hello ".simple_hi_blue(), "\x1b[94m hello \x1b[39m");
/// assert_eq!(" hello ".simple_hi_magenta(), "\x1b[95m hello \x1b[39m");
/// assert_eq!(" hello ".simple_hi_cyan(), "\x1b[96m hello \x1b[39m");
/// assert_eq!(" hello ".simple_hi_white(), "\x1b[97m hello \x1b[39m");
/// ```
pub trait SimpleForeground: Decorate {
    fn simple_black(&self) -> String {
        self.wrap(foreground::BLACK, cancel::FOREGROUND)
    }

    fn simple_red(&self) -> String {
        self.wrap(foreground::RED, cancel::FOREGROUND)
    }

    fn simple_green(&self) -> String {
        self.wrap(foreground::GREEN, cancel::FOREGROUND)
    }

    fn simple_yellow(&self) -> String {
        self.wrap(foreground::YELLOW, cancel::FOREGROUND)
    }

    fn simple_blue(&self) -> String {
        self.wrap(foreground::BLUE, cancel::FOREGROUND)
    }

    fn simple_magenta(&self) -> String {
        self.wrap(foreground::MAGENTA, cancel::FOREGROUND)
    }

    fn simple_cyan(&self) -> String {
        self.wrap(foreground::CYAN, cancel::FOREGROUND)
    }

    fn simple_white(&self) -> String {
        self.wrap(foreground::WHITE, cancel::FOREGROUND)
    }

    fn simple_hi_black(&self) -> String {
        self.wrap(foreground::BRIGHT_BLACK, cancel::FOREGROUND)
    }

    fn simple_hi_red(&self) -> String {
        self.wrap(foreground::BRIGHT_RED, cancel::FOREGROUND)
    }

    fn simple_hi_green(&self) -> String {
        self.wrap(foreground::BRIGHT_GREEN, cancel::FOREGROUND)
    }

    fn simple_hi_yellow(&self) -> String {
        self.wrap(foreground::BRIGHT_YELLOW, cancel::FOREGROUND)
    }

    fn simple_hi_blue(&self) -> String {
        self.wrap(foreground::BRIGHT_BLUE, cancel::FOREGROUND)
    }

    fn simple_hi_magenta(&self) -> String {
        self.wrap(foreground::BRIGHT_MAGENTA, cancel::FOREGROUND)
    }

    fn simple_hi_cyan(&self) -> String {
        self.wrap(foreground::BRIGHT_CYAN, cancel::FOREGROUND)
    }

    fn simple_hi_white(&self) -> String {
        self.wrap(foreground::BRIGHT_WHITE, cancel::FOREGROUND)
    }
}

impl<T: Decorate + AsRef<str>> SimpleStyle for T {}
impl<T: Decorate + AsRef<str>> SimpleForeground for T {}
