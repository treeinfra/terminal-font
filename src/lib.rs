pub mod custom_color;
pub mod decorate;
pub mod escape;
pub mod functions;
pub mod render;

pub use custom_color::*;
pub use decorate::*;
pub use escape::*;
pub use functions::{
    simple_background::*, simple_foreground::*, simple_style::*,
    simple_style_alias::*,
};
