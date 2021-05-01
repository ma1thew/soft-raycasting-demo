use crate::map::{Map, MapCell};
use crate::util::{Side, Step};
use crate::vec2::Vec2;

use minifb::{Key, Window};

pub struct Intersection {
    pub side: Side,
    pub step: Vec2<Step>,
    pub map_coordinates: Vec2<usize>,
    pub wall_offset: Vec2<f64>,
}

pub struct Ray {
    pub direction: Vec2<f64>,
    pub intersections: Vec<Intersection>,
}

pub struct Camera {
    pub position: Vec2<f64>,
    pub direction: Vec2<f64>,
    pub plane: Vec2<f64>,
    pub height: f64,
}

impl Camera {
    pub fn get_ray(&self, x: usize, screen_width: usize) -> Ray {
        let camera_x: f64 = 2.0 * (x as f64) / (screen_width as f64) - 1.0;
        Ray {
            direction: &self.direction + &self.plane * camera_x,
            intersections: Vec::new(),
        }
    }

    pub fn update_position_with_keys(&mut self, delta: f64, window: &Window, world: &Map) {
        let move_speed = delta * 5.0;
        let rot_speed = delta * 3.0;

        if window.is_key_down(Key::W) {
            if let Some(MapCell::Empty {
                ceiling_texture: _,
                floor_texture: _,
                fog: _,
                fog_color: _,
            }) = world.at(&(&self.position + &self.direction * move_speed).as_usize())
            {
                self.position += &self.direction * move_speed;
            }
        }
        if window.is_key_down(Key::S) {
            if let Some(MapCell::Empty {
                ceiling_texture: _,
                floor_texture: _,
                fog: _,
                fog_color: _,
            }) = world.at(&(&self.position - &self.direction * move_speed).as_usize())
            {
                self.position -= &self.direction * move_speed;
            }
        }
        if window.is_key_down(Key::A) {
            let mut direction = self.direction.clone();
            direction.rotate(-std::f64::consts::PI / 2.0);
            if let Some(MapCell::Empty {
                ceiling_texture: _,
                floor_texture: _,
                fog: _,
                fog_color: _,
            }) = world.at(&(&self.position - &direction * move_speed).as_usize())
            {
                self.position -= &direction * (move_speed / 1.5);
            }
        }
        if window.is_key_down(Key::D) {
            let mut direction = self.direction.clone();
            direction.rotate(std::f64::consts::PI / 2.0);
            if let Some(MapCell::Empty {
                ceiling_texture: _,
                floor_texture: _,
                fog: _,
                fog_color: _,
            }) = world.at(&(&self.position - &direction * move_speed).as_usize())
            {
                self.position -= &direction * (move_speed / 1.5);
            }
        }
        if window.is_key_down(Key::Left) {
            self.direction.rotate(rot_speed);
            self.plane.rotate(rot_speed);
        }
        if window.is_key_down(Key::Right) {
            self.direction.rotate(-rot_speed);
            self.plane.rotate(-rot_speed);
        }
    }
}
