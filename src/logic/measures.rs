use crate::config::Config;
use bevy::prelude::Component;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Display, PartialEq, Eq)]
pub enum GeometryType {
    Torus,
    FlatEarth,
    RingVertical,
    RingHorizontal,
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct VirtualCoords {
    pub x: i32,
    pub y: i32,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct RealCoords {
    pub x: u32,
    pub y: u32,
}

impl VirtualCoords {
    pub fn to_real(self, config: &Config) -> RealCoords {
        match config.map.geometry.value {
            GeometryType::Torus => {
                let x = wrap_value(0, self.x, config.map.size_x.value);
                let y = wrap_value(0, self.y, config.map.size_y.value);
                RealCoords { x, y }
            }
            GeometryType::FlatEarth => {
                let x = limit_value(0, self.x, 0, config.map.size_x.value);
                let y = limit_value(0, self.y, 0, config.map.size_y.value);
                RealCoords { x, y }
            }
            GeometryType::RingVertical => {
                let x = limit_value(0, self.x, 0, config.map.size_x.value);
                let y = wrap_value(0, self.y, config.map.size_y.value);
                RealCoords { x, y }
            }
            GeometryType::RingHorizontal => {
                let x = wrap_value(0, self.x, config.map.size_x.value);
                let y = limit_value(0, self.y, 0, config.map.size_y.value);
                RealCoords { x, y }
            }
        }
    }
}

pub fn wrap_value(x: u32, delta: i32, max: u32) -> u32 {
    let result_signed = x as i32 + delta;
    if result_signed >= 0 {
        (result_signed % (max as i32)) as u32
    } else {
        ((result_signed % (max as i32) + max as i32) % (max as i32)) as u32
    }
}

pub fn limit_value(x: u32, delta: i32, min: u32, max: u32) -> u32 {
    let x = x as i32 + delta;
    if x < min as i32 {
        min
    } else if x >= max as i32 {
        max - 1
    } else {
        x as u32
    }
}

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_wrap_value_correctly() {
        assert_eq!(wrap_value(0, -1, 10), 9);
        assert_eq!(wrap_value(9, 1, 10), 0);
    }

    #[quickcheck]
    fn should_limit_value_in_bounds(x: u32, delta: i32, min: u32, max: u32) -> bool {
        if min >= max || delta < -100 || delta > 100 || x >= max || x < min || max > 100 {
            return true;
        }
        println!("x: {}, delta: {}, min: {}, max: {}", x, delta, min, max);
        let result = limit_value(x, delta, min, max);
        result >= min && result < max
    }

    #[quickcheck]
    fn should_wrap_value_in_bounds(x: u32, delta: i32, max: u32) -> bool {
        if delta < -(max as i32) || delta > max as i32 || x >= max || max > 100 {
            return true;
        }
        let result = wrap_value(x, delta, max);
        result < max
    }
}
