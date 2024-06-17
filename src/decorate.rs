use crate::escape::cancel;
use crate::escape::style::*;

pub trait Decorate {
    /// Simply wrap a prefix and a suffix.
    ///
    /// This method is seldom directly used.
    /// You can find more encapsulations with escape codes
    /// in the [SimpleDecorateStyle] supertrait.
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
/// use terminal_font::decorate::SimpleDecorateStyle;
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
pub trait SimpleDecorateStyle: Decorate {
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

impl<T: Decorate + AsRef<str>> SimpleDecorateStyle for T {}
