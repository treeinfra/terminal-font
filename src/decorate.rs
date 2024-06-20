//! This mod only contains basic simple decoration functions and methods.
//! As for the customized colors, please refer to the [custom_color] module.
//! And as for the safe decorate, please refer to the [render] module.
//!
//! ## Render style risk and performance
//!
//! Once there's complex escape decorations inside a string,
//! there might be conflicts or overrides,
//! that just wrapping such string with corresponding escape codes
//! might made mistake styles.
//! Current mod doesn't concern such risk.
//! Because current mod is designed for better performance.
//! If you do need to prevent such risk, please refer to the [render] module.
//!
//! ## The `simple` name prefix
//!
//! All decoration functions and methods
//! with such risk will have a `simple` prefix.
//! If a function or method inside this mod doesn't has the `simple` prefix,
//! it means it's safe even just wrapping them
//! without parsing the inner structure.
//! This is guaranteed by the ansi escape rule.

use crate::escape::{background, cancel, foreground, style::*};

#[allow(unused_imports)] // Docs only.
use crate::{custom_color, render};

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
    ///
    /// ## Generics and performance
    ///
    /// The input parameters `prefix` and `suffix`, and even the `self`,
    /// can either be &[str] or [String] ([AsRef]<[str]>).
    /// Such generics will not have bad effect on performance
    /// because when formatting, a [String] will be converted into &[str].
    /// And when calling [AsRef::as_ref] on &[str],
    /// it will just return itself, and such process will be optimized
    /// by the compiler, especially in production mode.
    ///
    /// ```rust
    /// use terminal_font::decorate::Decorate;
    /// let prefix = "[";
    /// let suffix = "]";
    /// let prefix_string = String::from(prefix);
    /// let suffix_string = String::from(suffix);
    /// assert_eq!("hello".wrap(prefix, suffix), "[hello]");
    /// assert_eq!("hello".wrap(prefix, &suffix_string), "[hello]");
    /// assert_eq!("hello".wrap(&prefix_string, suffix), "[hello]");
    /// // assert_eq!("hello".wrap(prefix_string, suffix_string), "[hello]");
    /// ```
    fn wrap<T: AsRef<str>, U: AsRef<str>>(
        &self,
        prefix: T,
        suffix: U,
    ) -> String;
}

impl<T: AsRef<str>> Decorate for T {
    fn wrap<U: AsRef<str>, V: AsRef<str>>(
        &self,
        prefix: U,
        suffix: V,
    ) -> String {
        format!("{}{}{}", prefix.as_ref(), self.as_ref(), suffix.as_ref())
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
/// assert_eq!(" hello ".italic(), "\x1b[3m hello \x1b[23m");
/// assert_eq!(" hello ".simple_underline(), "\x1b[4m hello \x1b[24m");
/// assert_eq!(" hello ".simple_blink(), "\x1b[5m hello \x1b[25m");
/// assert_eq!(" hello ".simple_blink_fast(), "\x1b[6m hello \x1b[25m");
/// assert_eq!(" hello ".inverse(), "\x1b[7m hello \x1b[27m");
/// assert_eq!(" hello ".conceal(), "\x1b[8m hello \x1b[28m");
/// assert_eq!(" hello ".strikethrough(), "\x1b[9m hello \x1b[29m");
/// assert_eq!(" hello ".simple_double_underline(), "\x1b[21m hello \x1b[24m");
/// ```
pub trait SimpleStyle: Decorate {
    /// See the documentation of the trait: [SimpleStyle].
    fn simple_bold(&self) -> String {
        self.wrap(BOLD, cancel::BOLD_OR_FAINT)
    }

    /// Also known as dim.
    /// See the documentation of the trait: [SimpleStyle].
    fn simple_faint(&self) -> String {
        self.wrap(FAINT, cancel::BOLD_OR_FAINT)
    }

    /// Also known as oblique.
    /// See the documentation of the trait: [SimpleStyle].
    fn italic(&self) -> String {
        self.wrap(ITALIC, cancel::ITALIC)
    }

    /// See the documentation of the trait: [SimpleStyle].
    fn simple_underline(&self) -> String {
        self.wrap(UNDERLINE, cancel::UNDERLINE)
    }

    /// Usually unsupported by build-in terminals of common editors.
    /// See the documentation of the trait: [SimpleStyle].
    fn simple_blink(&self) -> String {
        self.wrap(BLINK, cancel::BLINK)
    }

    /// Usually unsupported by common terminals.
    /// See the documentation of the trait: [SimpleStyle].
    fn simple_blink_fast(&self) -> String {
        self.wrap(BLINK_FAST, cancel::BLINK)
    }

    /// Also known as negative.
    /// See the documentation of the trait: [SimpleStyle].
    fn inverse(&self) -> String {
        self.wrap(INVERSE, cancel::INVERSE)
    }

    /// Also known as hidden.
    /// See the documentation of the trait: [SimpleStyle].
    fn conceal(&self) -> String {
        self.wrap(CONCEAL, cancel::CONCEAL)
    }

    /// Also known as delete line.
    /// See the documentation of the trait: [SimpleStyle].
    fn strikethrough(&self) -> String {
        self.wrap(STRIKETHROUGH, cancel::STRIKETHROUGH)
    }

    /// Usually unsupported by common terminals,
    /// and sometimes displayed as a thick underline.
    /// See the documentation of the trait: [SimpleStyle].
    fn simple_double_underline(&self) -> String {
        self.wrap(DOUBLE_UNDERLINE, cancel::UNDERLINE)
    }
}

/// Aliases for some of the methods in the [SimpleStyle] trait.
///
/// ```rust
/// use terminal_font::decorate::SimpleStyleAlias;
/// assert_eq!(" hello ".simple_dim(), "\x1b[2m hello \x1b[22m");
/// assert_eq!(" hello ".oblique(), "\x1b[3m hello \x1b[23m");
/// assert_eq!(" hello ".negative(), "\x1b[7m hello \x1b[27m");
/// assert_eq!(" hello ".hidden(), "\x1b[8m hello \x1b[28m");
/// assert_eq!(" hello ".delete_line(), "\x1b[9m hello \x1b[29m");
/// ```
pub trait SimpleStyleAlias: SimpleStyle {
    /// Alias of [SimpleStyle::simple_faint].
    fn simple_dim(&self) -> String {
        self.simple_faint()
    }

    /// Alias of [SimpleStyle::italic].
    fn oblique(&self) -> String {
        self.italic()
    }

    /// Alias of [SimpleStyle::inverse].
    fn negative(&self) -> String {
        self.inverse()
    }

    /// Alias of [SimpleStyle::conceal].
    fn hidden(&self) -> String {
        self.conceal()
    }

    /// Alias of [SimpleStyle::strikethrough].
    fn delete_line(&self) -> String {
        self.strikethrough()
    }
}

/// Encapsulation on the [Decorate] trait with
/// commonly used escape codes for foreground color decorations.
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

/// Encapsulation on the [Decorate] trait with
/// commonly used escape codes for background color decorations.
///
/// Those methods won't check whether there are conflicts in the raw string.
/// The only thing to do is wrap the prefix and the suffix.
/// Those methods are usually used when content is predicated,
/// and performance is more important.
///
/// ```rust
/// use terminal_font::decorate::SimpleBackground;
/// assert_eq!(" hello ".simple_bg_black(), "\x1b[40m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_red(), "\x1b[41m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_green(), "\x1b[42m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_yellow(), "\x1b[43m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_blue(), "\x1b[44m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_magenta(), "\x1b[45m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_cyan(), "\x1b[46m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_white(), "\x1b[47m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_hi_black(), "\x1b[100m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_hi_red(), "\x1b[101m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_hi_green(), "\x1b[102m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_hi_yellow(), "\x1b[103m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_hi_blue(), "\x1b[104m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_hi_magenta(), "\x1b[105m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_hi_cyan(), "\x1b[106m hello \x1b[49m");
/// assert_eq!(" hello ".simple_bg_hi_white(), "\x1b[107m hello \x1b[49m");
/// ```
pub trait SimpleBackground: Decorate {
    fn simple_bg_black(&self) -> String {
        self.wrap(background::BLACK, cancel::BACKGROUND)
    }

    fn simple_bg_red(&self) -> String {
        self.wrap(background::RED, cancel::BACKGROUND)
    }

    fn simple_bg_green(&self) -> String {
        self.wrap(background::GREEN, cancel::BACKGROUND)
    }

    fn simple_bg_yellow(&self) -> String {
        self.wrap(background::YELLOW, cancel::BACKGROUND)
    }

    fn simple_bg_blue(&self) -> String {
        self.wrap(background::BLUE, cancel::BACKGROUND)
    }

    fn simple_bg_magenta(&self) -> String {
        self.wrap(background::MAGENTA, cancel::BACKGROUND)
    }

    fn simple_bg_cyan(&self) -> String {
        self.wrap(background::CYAN, cancel::BACKGROUND)
    }

    fn simple_bg_white(&self) -> String {
        self.wrap(background::WHITE, cancel::BACKGROUND)
    }

    fn simple_bg_hi_black(&self) -> String {
        self.wrap(background::BRIGHT_BLACK, cancel::BACKGROUND)
    }

    fn simple_bg_hi_red(&self) -> String {
        self.wrap(background::BRIGHT_RED, cancel::BACKGROUND)
    }

    fn simple_bg_hi_green(&self) -> String {
        self.wrap(background::BRIGHT_GREEN, cancel::BACKGROUND)
    }

    fn simple_bg_hi_yellow(&self) -> String {
        self.wrap(background::BRIGHT_YELLOW, cancel::BACKGROUND)
    }

    fn simple_bg_hi_blue(&self) -> String {
        self.wrap(background::BRIGHT_BLUE, cancel::BACKGROUND)
    }

    fn simple_bg_hi_magenta(&self) -> String {
        self.wrap(background::BRIGHT_MAGENTA, cancel::BACKGROUND)
    }

    fn simple_bg_hi_cyan(&self) -> String {
        self.wrap(background::BRIGHT_CYAN, cancel::BACKGROUND)
    }

    fn simple_bg_hi_white(&self) -> String {
        self.wrap(background::BRIGHT_WHITE, cancel::BACKGROUND)
    }
}

impl<T: Decorate + AsRef<str>> SimpleStyle for T {}
impl<T: Decorate + AsRef<str>> SimpleStyleAlias for T {}
impl<T: Decorate + AsRef<str>> SimpleForeground for T {}
impl<T: Decorate + AsRef<str>> SimpleBackground for T {}
