use std::fmt::Debug;
use std::hash::Hash;

pub trait ColorContainer:
    Sized + Debug + Send + Sync + Clone + Copy + PartialEq + Eq + Hash
{
    fn is_black(&self) -> bool;
}

pub trait RgbContainer: ColorContainer {
    fn get_rgb(&self) -> RGB;
    fn copy_with_new_rgb(&self, rgb: RGB) -> Self;
}

pub trait RgbwContainer: ColorContainer {
    fn get_rgbw(&self) -> RGBW;
    fn copy_with_new_rgbw(&self, rgb: RGBW) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RGBW {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub white: u8,
}

impl ColorContainer for RGB {
    fn is_black(&self) -> bool {
        self.red == 0 && self.green == 0 && self.blue == 0
    }
}

impl RgbContainer for RGB {
    fn get_rgb(&self) -> RGB {
        *self
    }
    fn copy_with_new_rgb(&self, rgb: RGB) -> Self {
        RGB {
            red: rgb.red,
            green: rgb.green,
            blue: rgb.blue,
        }
    }
}

impl ColorContainer for RGBW {
    fn is_black(&self) -> bool {
        self.red == 0 && self.green == 0 && self.blue == 0 && self.white == 0
    }
}

impl RgbwContainer for RGBW {
    fn get_rgbw(&self) -> RGBW {
        *self
    }
    fn copy_with_new_rgbw(&self, rgbw: RGBW) -> Self {
        RGBW {
            red: rgbw.red,
            green: rgbw.green,
            blue: rgbw.blue,
            white: rgbw.white,
        }
    }
}
