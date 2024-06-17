/// Generate ANSI escape sequence according to the given parameters.
/// You can either use it with a single parameter or multiple parameters:
///
/// ```rust
/// use terminal_font::escape;
///
/// assert_eq!(escape!(123), "\x1b[123m");
/// assert_eq!(escape!(38, 2, 123), "\x1b[38;2;123m");
/// assert_eq!(escape!(38, 5, 143, 76, 76), "\x1b[38;5;143;76;76m");
/// ```
///
/// You can even use it with strings (`str`) rather than integers:
///
/// ```rust
/// use terminal_font::escape;
///
/// assert_eq!(escape!("123"), "\x1b[123m");
/// assert_eq!(escape!("38", "2", "123"), "\x1b[38;2;123m");
/// ```
///
/// Or mix them together:
///
/// ```rust
/// use terminal_font::escape;
///
/// assert_eq!(escape!("38", 2, "123"), "\x1b[38;2;123m");
/// assert_eq!(escape!(38, "5", "143", 76, "76"), "\x1b[38;5;143;76;76m");
/// ```
///
/// ## Different return value and performance
///
/// 1. It will return a `str` if there's only a single parameter,
///    and the parameter is a static literal.
/// 2. It will return a `String` if there's more than one parameter,
///    or a single parameter but not a static literal.
///
/// This is designed to make commonly used escape sequences static
/// to improve performance, while keeping the flexibility for other conditions.
/// This marco will detect the parameter type and match them in compile time,
/// so you don't need to worry about the performance while coding.
///
/// ```rust
/// use std::any::{Any, TypeId};
/// use terminal_font::escape;
///
/// assert_eq!(escape!(123).type_id(), TypeId::of::<str>());
/// assert_eq!(escape!(123, 456).type_id(), TypeId::of::<String>());
/// for i in 0..5 {
///     let demo = escape!(i);
///     assert_eq!(demo.type_id(), TypeId::of::<String>());
/// }
/// ```
#[macro_export]
macro_rules! escape {
    ($param:literal) => {concat!("\x1b[", $param, "m")};
    ($param:expr) => {format!("\x1b[{}m", $param)};
    ($first:expr, $($rest:expr),+) => {{
        let mut handler = String::from("\x1b[");
        handler.push_str($first.to_string().as_str());
        $(
            handler.push_str(";");
            handler.push_str($rest.to_string().as_str());
        )+
        handler.push_str("m");
        handler
    }};
}

/// Cancel decoration escape codes, usually used as suffix.
pub mod cancel {
    pub const ALL: &str = escape!(0);
    pub const BOLD_OR_FAINT: &str = escape!(22);
    pub const ITALIC: &str = escape!(23);
    pub const UNDERLINE: &str = escape!(24);
    pub const BLINK: &str = escape!(25);
    pub const INVERSE: &str = escape!(27);
    pub const CONCEAL: &str = escape!(28);
    pub const STRIKETHROUGH: &str = escape!(29);
    pub const FOREGROUND: &str = escape!(39);
    pub const BACKGROUND: &str = escape!(49);
}

/// Escape code for font styles.
pub mod style {
    pub const BOLD: &str = escape!(1);
    pub const FAINT: &str = escape!(2);
    pub const ITALIC: &str = escape!(3);
    pub const UNDERLINE: &str = escape!(4);
    pub const BLINK: &str = escape!(5);
    pub const BLINK_FAST: &str = escape!(6);
    pub const INVERSE: &str = escape!(7);
    pub const CONCEAL: &str = escape!(8);
    pub const STRIKETHROUGH: &str = escape!(9);
    pub const DOUBLE_UNDERLINE: &str = escape!(21);
}

/// Escape code for foreground colors.
pub mod foreground {
    pub const BLACK: &str = escape!(30);
    pub const RED: &str = escape!(31);
    pub const GREEN: &str = escape!(32);
    pub const YELLOW: &str = escape!(33);
    pub const BLUE: &str = escape!(34);
    pub const MAGENTA: &str = escape!(35);
    pub const CYAN: &str = escape!(36);
    pub const WHITE: &str = escape!(37);

    pub const BRIGHT_BLACK: &str = escape!(90);
    pub const BRIGHT_RED: &str = escape!(91);
    pub const BRIGHT_GREEN: &str = escape!(92);
    pub const BRIGHT_YELLOW: &str = escape!(93);
    pub const BRIGHT_BLUE: &str = escape!(94);
    pub const BRIGHT_MAGENTA: &str = escape!(95);
    pub const BRIGHT_CYAN: &str = escape!(96);
    pub const BRIGHT_WHITE: &str = escape!(97);
}

/// Escape code for background colors.
pub mod background {
    pub const BLACK: &str = escape!(40);
    pub const RED: &str = escape!(41);
    pub const GREEN: &str = escape!(42);
    pub const YELLOW: &str = escape!(43);
    pub const BLUE: &str = escape!(44);
    pub const MAGENTA: &str = escape!(45);
    pub const CYAN: &str = escape!(46);
    pub const WHITE: &str = escape!(47);

    pub const BRIGHT_BLACK: &str = escape!(100);
    pub const BRIGHT_RED: &str = escape!(101);
    pub const BRIGHT_GREEN: &str = escape!(102);
    pub const BRIGHT_YELLOW: &str = escape!(103);
    pub const BRIGHT_BLUE: &str = escape!(104);
    pub const BRIGHT_MAGENTA: &str = escape!(105);
    pub const BRIGHT_CYAN: &str = escape!(106);
    pub const BRIGHT_WHITE: &str = escape!(107);
}
