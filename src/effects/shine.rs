use std::collections::HashSet;

use rand::prelude::{IteratorRandom, SliceRandom};
use rand::thread_rng;
use thiserror::Error;

use crate::util::color::{RgbContainer, RGB};
use crate::util::color_point::ColorPointContainer;
use crate::util::effect::Effect;

#[derive(Error, Debug)]
pub enum ShineError {
    #[error(
        "num_start_simultaneous must be between 1 up to and including the total number of points"
    )]
    InvalidNumStartSimultaneous,
    #[error("colors set must not be empty")]
    ColorSetIsEmpty,
}

pub struct Shine<U: RgbContainer, T: ColorPointContainer<U>> {
    points: Vec<T>,
    colors: HashSet<U>,
    frames_between_glow_start: u32,
    frames_to_max_glow: u32,
    frames_to_fade: u32,
    num_start_simultaneous: usize,
    current_frame: u64,
    glow_start_times: Vec<u64>,
    frames_since_last_glow: u32,
    current_glow_colors: Vec<U>,
}

impl<U: RgbContainer, T: ColorPointContainer<U>> Shine<U, T> {
    pub fn new(
        points: Vec<T>,
        colors: HashSet<U>,
        frames_between_glow_start: u32,
        frames_to_max_glow: u32,
        frames_to_fade: u32,
        num_start_simultaneous: usize,
    ) -> Result<Self, ShineError> {
        if num_start_simultaneous == 0 || num_start_simultaneous > points.len() {
            return Err(ShineError::InvalidNumStartSimultaneous);
        }
        if colors.is_empty() {
            return Err(ShineError::ColorSetIsEmpty);
        }
        let current_frame = ((frames_to_max_glow + frames_to_fade) * 2) as u64;

        let glow_start_times = vec![0_u64; points.len()];

        let frames_since_last_glow = frames_between_glow_start;
        let current_glow_colors = vec![*colors.iter().nth(0).unwrap(); points.len()];

        Ok(Shine {
            points,
            colors,
            frames_between_glow_start,
            frames_to_max_glow,
            frames_to_fade,
            num_start_simultaneous,
            current_frame,
            glow_start_times,
            frames_since_last_glow,
            current_glow_colors,
        })
    }
}

impl<U: RgbContainer, T: ColorPointContainer<U>> Effect<U, T> for Shine<U, T> {
    fn get_frame(&mut self) -> Vec<T> {
        let num_points = self.points.len();
        if self.frames_since_last_glow >= self.frames_between_glow_start {
            let mut rng = thread_rng();
            let mut available_points: Vec<usize> = (0..num_points)
                .filter(|&i| {
                    self.current_frame - self.glow_start_times[i]
                        >= (self.frames_to_max_glow + self.frames_to_fade) as u64
                })
                .collect();
            available_points.shuffle(&mut rng);
            for &led_index in available_points.iter().take(self.num_start_simultaneous) {
                self.glow_start_times[led_index] = self.current_frame;
                // Randomly select a color for the LED
                self.points[led_index] = self
                    .colors
                    .iter()
                    .map(|color| self.points[led_index].copy_with_new_color_value(*color))
                    .choose(&mut rng)
                    .expect("colors set is empty");
            }
            self.frames_since_last_glow = 0;
        } else {
            self.frames_since_last_glow += 1;
        }

        for i in 0..num_points {
            let elapsed = self.current_frame - self.glow_start_times[i];

            let glow_color = if self.points[i].get_color_value().is_black() {
                self.current_glow_colors[i] =
                    *self.colors.iter().choose(&mut thread_rng()).unwrap();
                self.current_glow_colors[i]
            } else {
                self.current_glow_colors[i]
            };

            let brightness = if elapsed < self.frames_to_max_glow as u64 {
                elapsed as f64 / self.frames_to_max_glow as f64
            } else if elapsed < (self.frames_to_max_glow + self.frames_to_fade) as u64 {
                1.0 - (elapsed - self.frames_to_max_glow as u64) as f64 / self.frames_to_fade as f64
            } else {
                0.0
            };

            self.points[i] = self.points[i].copy_with_new_color_value(
                self.points[i].get_color_value().copy_with_new_rgb(RGB {
                    red: (glow_color.get_rgb().red as f64 * brightness) as u8,
                    green: (glow_color.get_rgb().green as f64 * brightness) as u8,
                    blue: (glow_color.get_rgb().blue as f64 * brightness) as u8,
                }),
            );
        }
        self.current_frame += 1;
        self.points.clone()
    }
}

#[cfg(test)]
mod shine_tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    use crate::util::color_point::RgbPoint;
    use crate::util::point::Point;

    use super::*;

    // Assuming this creates a Shine instance for testing with RGB and RgbPoint
    fn create_test_shine() -> Shine<RGB, RgbPoint<RGB>> {
        let points = vec![
            RgbPoint {
                point: Point {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                color: RGB {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
            },
            RgbPoint {
                point: Point {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                color: RGB {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
            },
        ];
        let colors = HashSet::from_iter(vec![RGB {
            red: 255,
            green: 0,
            blue: 0,
        }]);
        Shine::new(points, colors, 1, 2, 2, 1).expect("Failed to create Shine")
    }

    #[test]
    fn test_shade_of_red_with_no_green_or_blue() {
        let mut shine = create_test_shine();

        // Run several frames to get past initial states and into a cyclic glow state
        for _ in 0..5 {
            shine.get_frame();
        }

        // Check after several frames that at least one point's color is a shade of red
        // with no green or blue components (indicating it's indeed glowing red).
        let points = shine.get_frame(); // Assume this gets the current state of all points

        let mut found_shade_of_red = false;
        for point in points.iter() {
            let color = point.get_color_value();
            if color.red > 0 && color.green == 0 && color.blue == 0 {
                found_shade_of_red = true;
                break;
            }
        }

        assert!(found_shade_of_red, "Failed to find at least one point that is a shade of red with no green or blue elements.");
    }
}
