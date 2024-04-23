use std::fmt::Debug;

pub trait PointContainer: Sized + Debug + Send + Sync + Clone + Copy {
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
    fn get_z(&self) -> f64;
    fn copy_with_new_coordinates(&self, x: f64, y: f64, z: f64) -> Self;
    fn get_nalgebra_point(&self) -> nalgebra::Point3<f64> {
        nalgebra::Point3::new(self.get_x(), self.get_y(), self.get_z())
    }
}

impl PointContainer for nalgebra::Point3<f64> {
    fn get_x(&self) -> f64 {
        self.x
    }
    fn get_y(&self) -> f64 {
        self.y
    }

    fn get_z(&self) -> f64 {
        self.z
    }

    fn copy_with_new_coordinates(&self, x: f64, y: f64, z: f64) -> Self {
        nalgebra::Point3::new(x, y, z)
    }
}

impl PointContainer for Point {
    fn get_x(&self) -> f64 {
        self.x
    }
    fn get_y(&self) -> f64 {
        self.y
    }

    fn get_z(&self) -> f64 {
        self.z
    }

    fn copy_with_new_coordinates(&self, x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
