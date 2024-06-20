//! Function encapsulations for [decorate] and [render].
//! It's usually used when you need something like a colorizer.

#[allow(unused_imports)] // Docs only.
use crate::{decorate, render};

pub mod simple_style {
    #[allow(unused_imports)] // Docs only.
    use crate::decorate;
    use crate::decorate::*;

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bold<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bold()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_faint<T: AsRef<str>>(raw: T) -> String {
        raw.simple_faint()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn italic<T: AsRef<str>>(raw: T) -> String {
        raw.italic()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_underline<T: AsRef<str>>(raw: T) -> String {
        raw.simple_underline()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_blink<T: AsRef<str>>(raw: T) -> String {
        raw.simple_blink()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_blink_fast<T: AsRef<str>>(raw: T) -> String {
        raw.simple_blink_fast()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn inverse<T: AsRef<str>>(raw: T) -> String {
        raw.inverse()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn conceal<T: AsRef<str>>(raw: T) -> String {
        raw.conceal()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn strikethrough<T: AsRef<str>>(raw: T) -> String {
        raw.strikethrough()
    }
}

pub mod simple_style_alias {
    #[allow(unused_imports)] // Docs only.
    use crate::decorate;
    use crate::decorate::*;

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_dim<T: AsRef<str>>(raw: T) -> String {
        raw.simple_dim()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn oblique<T: AsRef<str>>(raw: T) -> String {
        raw.oblique()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn negative<T: AsRef<str>>(raw: T) -> String {
        raw.negative()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn hidden<T: AsRef<str>>(raw: T) -> String {
        raw.hidden()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn delete_line<T: AsRef<str>>(raw: T) -> String {
        raw.delete_line()
    }
}

pub mod simple_foreground {
    #[allow(unused_imports)] // Docs only.
    use crate::decorate;
    use crate::decorate::*;

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_black<T: AsRef<str>>(raw: T) -> String {
        raw.simple_black()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_red<T: AsRef<str>>(raw: T) -> String {
        raw.simple_red()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_green<T: AsRef<str>>(raw: T) -> String {
        raw.simple_green()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_yellow<T: AsRef<str>>(raw: T) -> String {
        raw.simple_yellow()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_blue<T: AsRef<str>>(raw: T) -> String {
        raw.simple_blue()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_magenta<T: AsRef<str>>(raw: T) -> String {
        raw.simple_magenta()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_cyan<T: AsRef<str>>(raw: T) -> String {
        raw.simple_cyan()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_white<T: AsRef<str>>(raw: T) -> String {
        raw.simple_white()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_hi_black<T: AsRef<str>>(raw: T) -> String {
        raw.simple_hi_black()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_hi_red<T: AsRef<str>>(raw: T) -> String {
        raw.simple_hi_red()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_hi_green<T: AsRef<str>>(raw: T) -> String {
        raw.simple_hi_green()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_hi_yellow<T: AsRef<str>>(raw: T) -> String {
        raw.simple_hi_yellow()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_hi_blue<T: AsRef<str>>(raw: T) -> String {
        raw.simple_hi_blue()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_hi_magenta<T: AsRef<str>>(raw: T) -> String {
        raw.simple_hi_magenta()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_hi_cyan<T: AsRef<str>>(raw: T) -> String {
        raw.simple_hi_cyan()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_hi_white<T: AsRef<str>>(raw: T) -> String {
        raw.simple_hi_white()
    }
}

pub mod simple_background {
    #[allow(unused_imports)] // Docs only.
    use crate::decorate;
    use crate::decorate::*;

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_black<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_black()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_red<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_red()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_green<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_green()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_yellow<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_yellow()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_blue<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_blue()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_magenta<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_magenta()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_cyan<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_cyan()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_white<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_white()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_hi_black<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_hi_black()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_hi_red<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_hi_red()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_hi_green<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_hi_green()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_hi_yellow<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_hi_yellow()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_hi_blue<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_hi_blue()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_hi_magenta<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_hi_magenta()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_hi_cyan<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_hi_cyan()
    }

    /// See [decorate] about what does `simple` name prefix means.
    pub fn simple_bg_hi_white<T: AsRef<str>>(raw: T) -> String {
        raw.simple_bg_hi_white()
    }
}
