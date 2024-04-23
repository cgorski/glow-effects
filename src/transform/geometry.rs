use std::f64::consts::PI;

use nalgebra::{Rotation3, Vector3};
use thiserror::Error;

use crate::util::point::PointContainer;

// Ensure this is correctly defined as shown above

#[derive(Debug, Clone, Copy)]
pub enum Transformation {
    FlipAcrossXAxis,
    FlipAcrossYAxis,
    FlipAcrossZAxis,
    RotateAroundAxis { axis: Axis, angle_degrees: f64 },
}

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Error, Debug)]
pub enum TransformationError {
    #[error("Invalid rotation axis or angle")]
    InvalidRotation,
}

pub trait Transform<T: PointContainer>: Sized {
    fn apply_transformation(
        points: Self,
        transformation: Transformation,
    ) -> Result<Self, TransformationError>;
}

// implement the Transformer trait for Vec<T>
impl<T: PointContainer> Transform<T> for Vec<T> {
    fn apply_transformation(
        points: Vec<T>,
        transformation: Transformation,
    ) -> Result<Vec<T>, TransformationError> {
        apply_transformation_helper(points, transformation)
    }
}

fn apply_transformation_helper<T: PointContainer>(
    points: Vec<T>,
    transformation: Transformation,
) -> Result<Vec<T>, TransformationError> {
    let transform_matrix = match transformation {
        Transformation::FlipAcrossXAxis => Rotation3::from_axis_angle(&Vector3::x_axis(), PI),
        Transformation::FlipAcrossYAxis => Rotation3::from_axis_angle(&Vector3::y_axis(), PI),
        Transformation::FlipAcrossZAxis => Rotation3::from_axis_angle(&Vector3::z_axis(), PI),
        Transformation::RotateAroundAxis {
            axis,
            angle_degrees,
        } => {
            let rad = angle_degrees.to_radians();
            match axis {
                Axis::X => Rotation3::from_axis_angle(&Vector3::x_axis(), rad),
                Axis::Y => Rotation3::from_axis_angle(&Vector3::y_axis(), rad),
                Axis::Z => Rotation3::from_axis_angle(&Vector3::z_axis(), rad),
            }
        }
    };

    let transformed_points: Vec<T> = points
        .iter()
        .map(|point| {
            let original_point = point.get_nalgebra_point();
            let new_point = transform_matrix * original_point;
            point.copy_with_new_coordinates(new_point.x, new_point.y, new_point.z)
        })
        .collect();

    Ok(transformed_points)
}
