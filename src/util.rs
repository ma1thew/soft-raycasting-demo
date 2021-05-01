use std::rc::Rc;

use crate::vec2::Vec2;
use crate::texture::Texture;

#[derive(Clone)]
pub enum Side {
    X,
    Y,
}

pub enum Orientation {
    XAxis,
    YAxis,
}

#[derive(Clone)]
pub enum Step {
    Left,
    Right,
}

impl Step {
    pub fn value(&self) -> i32 {
        match self {
            Step::Left => -1,
            Step::Right => 1,
        }
    }

    pub fn from(value: bool) -> Step {
        match value {
            true => Step::Left,
            false => Step::Right,
        }
    }
}

pub struct Sprite {
    pub position: Vec2<f64>,
    pub texture: Rc<Texture>,
    pub vertical_offset: f64,
    pub scale_factor: Vec2<f64>,
    pub distance_from_camera: f64,
}
