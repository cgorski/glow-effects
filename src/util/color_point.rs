use crate::util::color::{ColorContainer, RgbContainer, RgbwContainer, RGB};
use crate::util::point::{Point, PointContainer};

pub trait ColorPointContainer<T: ColorContainer>: PointContainer {
    fn copy_with_new_color_value(&self, color_value: T) -> Self;
    fn get_color_value(&self) -> T;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbPoint<T: RgbContainer> {
    pub point: Point,
    pub color: T,
}

impl RgbPoint<RGB> {
    pub fn new(point: Point, color: RGB) -> Self {
        RgbPoint { point, color }
    }
}

impl<T: RgbContainer> PointContainer for RgbPoint<T> {
    fn get_x(&self) -> f64 {
        self.point.x
    }
    fn get_y(&self) -> f64 {
        self.point.y
    }

    fn get_z(&self) -> f64 {
        self.point.z
    }

    fn copy_with_new_coordinates(&self, x: f64, y: f64, z: f64) -> Self {
        RgbPoint {
            point: Point { x, y, z },
            color: self.color,
        }
    }
}

impl<T: RgbContainer> ColorPointContainer<T> for RgbPoint<T> {
    fn copy_with_new_color_value(&self, color_value: T) -> Self {
        RgbPoint {
            point: self.point,
            color: self.color.copy_with_new_rgb(color_value.get_rgb()),
        }
    }
    fn get_color_value(&self) -> T {
        self.color
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbwPoint<T: RgbwContainer> {
    pub point: Point,
    pub color_value: T,
}

impl<T: RgbwContainer> PointContainer for RgbwPoint<T> {
    fn get_x(&self) -> f64 {
        self.point.x
    }
    fn get_y(&self) -> f64 {
        self.point.y
    }

    fn get_z(&self) -> f64 {
        self.point.z
    }

    fn copy_with_new_coordinates(&self, x: f64, y: f64, z: f64) -> Self {
        RgbwPoint {
            point: Point { x, y, z },
            color_value: self.color_value,
        }
    }
}

impl<T: RgbwContainer> ColorPointContainer<T> for RgbwPoint<T> {
    fn copy_with_new_color_value(&self, color_value: T) -> Self {
        RgbwPoint {
            point: self.point,
            color_value: self.color_value.copy_with_new_rgbw(color_value.get_rgbw()),
        }
    }
    fn get_color_value(&self) -> T {
        self.color_value
    }
}
