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
