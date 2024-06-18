use crate::{
    cancel,
    custom::{background_rgb, foreground_rgb},
    decorate::Decorate,
    escape,
};

pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub trait SimpleCustomColor: Decorate {
    fn rgb(&self, color: RGBColor) -> String {
        self.wrap(
            foreground_rgb(color.r, color.g, color.b),
            escape!(cancel::FOREGROUND),
        )
    }

    fn bg_rgb(&self, color: RGBColor) -> String {
        self.wrap(
            background_rgb(color.r, color.g, color.b),
            escape!(cancel::BACKGROUND),
        )
    }
}

impl<T: Decorate + AsRef<str>> SimpleCustomColor for T {}
