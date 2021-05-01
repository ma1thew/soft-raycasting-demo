use crate::texture::Font;
use crate::map::{MapCell, Map};
use crate::camera::{Ray, Camera};
use crate::util::{Side, Sprite};
use crate::vec2::Vec2;

pub struct Framebuffer {
    pub height: usize,
    pub width: usize,
    pub pixels: Vec<u32>,
    pub z_buffer: Vec<f64>,
}

impl Framebuffer {
    pub fn new(height: usize, width: usize) -> Framebuffer {
        Framebuffer { 
            height,
            width,
            pixels: vec![0; height * width],
            z_buffer: vec![f64::INFINITY; height * width],
        }
    }

    pub fn clear(&mut self) {
        for pixel in &mut self.pixels {
            *pixel = 0;
        }
    }

    pub fn clear_z_buffer(&mut self) {
        for pixel in &mut self.z_buffer {
            *pixel = f64::INFINITY;
        }
    }

    pub fn draw_vertical_line(&mut self, x: usize, start: usize, stop: usize, color: u32) {
        for row in start..stop {
            self.pixels[row * self.width + x] = color;
        }
    }

    pub fn write_ascii_string(&mut self, x: usize, y: usize, string: &Vec<u8>, font: &Font, color: u32) {
        for char_index in 0..string.len() {
            for glyph_x in 0..font.glyph_size {
                for glyph_y in 0..font.glyph_size {
                    if font.glyphs[(font.glyph_size - 1 - glyph_y) * (font.charset_length * font.glyph_size) + glyph_x + font.glyph_size * (string[char_index] as usize)] {
                        self.pixels[((y + glyph_y) * self.width) +  x + (char_index * font.glyph_size) + glyph_x] = color;
                    }
                }
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if let Some(pixel) = self.pixels.get_mut(y * self.width + x) {
            *pixel = color;
        }
    }

    pub fn draw_wall(&mut self, camera: &Camera, x: usize, perp_wall_dist: f64, cell: &MapCell, side: &Side, ray: &Ray, world: &Map) {
        let line_height = (self.height as f64 / perp_wall_dist) as i32;
        if line_height < 0 {
            self.draw_vertical_line(x, 0, self.height, 0x00FF0000);
            return;
        }
        let draw_start = ((-line_height / 2 + (self.height as i32) / 2) - 1 + ((camera.height / perp_wall_dist) as i32)).max(0);
        let draw_end = ((line_height / 2 + (self.height as i32) / 2) + 1 + ((camera.height / perp_wall_dist) as i32)).min(self.height as i32);

        match cell {
            MapCell::Wall { texture } | MapCell::ThinWall { texture, orientation: _, offset_into_cell: _, ceiling_texture: _, floor_texture: _ } => {
            let wall_x = match &side {
                Side::X => camera.position.y + perp_wall_dist * ray.direction.y,
                Side::Y=> camera.position.x + perp_wall_dist * ray.direction.x,
            }.fract();

            let mut tex_x = (wall_x * (texture.width as f64)) as usize;
            if let Side::X = side {
                if ray.direction.x > 0.0 {
                    tex_x = texture.width - tex_x - 1;
                }
            } else {
                if ray.direction.y < 0.0 {
                    tex_x = texture.width - tex_x - 1;
                }
            }
            let step = (texture.height as f64) / (line_height as f64);
            let mut tex_position = ((draw_start as f64) - (camera.height / perp_wall_dist) - (self.height as f64) / 2.0 + (line_height as f64) / 2.0) * step;
            for y in draw_start..draw_end {
                let tex_y = (tex_position as usize) & (texture.height - 1);
                tex_position += step;
                if perp_wall_dist < self.z_buffer[y as usize * self.width + x as usize] {
                    let mut color = texture.data[texture.height * tex_y + tex_x];
                    if (color & 0x00FFFFFF) != 0 {
                        if let Side::Y = side {
                            color = (color >> 1) & 8355711;
                        }
                        if let Some(MapCell::Empty { ceiling_texture: _, floor_texture: _, fog, fog_color }) = world.at(&camera.position.as_usize()) {
                            let fog_prop = (perp_wall_dist * fog).min(1.0);
                            if fog_prop > 0.0 {
                                let mut color_bytes = color.to_le_bytes();
                                let fog_bytes = fog_color.to_le_bytes();
                                color_bytes[0] = (fog_bytes[0] as f64 * fog_prop + color_bytes[0] as f64 * (1.0 - fog_prop)) as u8;
                                color_bytes[1] = (fog_bytes[1] as f64 * fog_prop + color_bytes[1] as f64 * (1.0 - fog_prop)) as u8;
                                color_bytes[2] = (fog_bytes[2] as f64 * fog_prop + color_bytes[2] as f64 * (1.0 - fog_prop)) as u8;
                                color = u32::from_le_bytes(color_bytes);
                            }
                        }
                        self.set_pixel(x, y as usize, color);
                        self.z_buffer[y as usize * self.width + x as usize] = perp_wall_dist;
                    }
                }
            }
            },
            MapCell::Empty { ceiling_texture: _, floor_texture: _, fog: _, fog_color: _ } => {},
        }
    }

    pub fn draw_sprites(&mut self, camera: &Camera, sprites: &mut Vec<Sprite>, world: &Map) {
        for sprite in sprites.into_iter() {
            sprite.distance_from_camera = (&camera.position - &sprite.position).length();
        }
        for sprite in sprites {
            let rel_position = &sprite.position - &camera.position;
            let inverse_det = 1.0 / (camera.plane.x * camera.direction.y - camera.direction.x * camera.plane.y);
            let transform = Vec2 {
                x: inverse_det * (camera.direction.y * rel_position.x - camera.direction.x * rel_position.y),
                y: inverse_det * (-camera.plane.y * rel_position.x + camera.plane.x * rel_position.y)
            };
            if transform.y == 0.0 {
                continue;
            }
            let vertical_offset = ((sprite.vertical_offset / transform.y) + (camera.height / transform.y)) as i32;
            let sprite_screen_x = ((self.width as f64 / 2.0) * (1.0 + transform.x / transform.y)) as i32;
            let sprite_height = (((self.height as f64 / transform.y) as i32).abs() as f64 * sprite.scale_factor.y) as i32;
            let sprite_width = (((self.height as f64 / transform.y) as i32).abs() as f64 * sprite.scale_factor.x) as i32;
            let draw_start = Vec2 {
                x: ((-sprite_width / 2) + sprite_screen_x).max(0),
                y: ((-sprite_height / 2 + (self.height as i32) / 2) + vertical_offset).max(0),
            };
            let draw_end = Vec2 {
                x: ((sprite_width / 2) + sprite_screen_x).min(self.width as i32),
                y: ((sprite_height / 2 + (self.height as i32) / 2) + vertical_offset).min(self.height as i32),
            };
            for column in draw_start.x..draw_end.x {
                let tex_x = (256 * (column - (-sprite_width / 2 + (sprite_screen_x))) * sprite.texture.width as i32 / sprite_width) as i32 / 256;
                if transform.y > 0.0 && column >= 0 && column < self.width as i32 {
                    for y in draw_start.y..draw_end.y {
                        let d = (y - vertical_offset) * 256 - self.height as i32 * 128 + sprite_height * 128;
                        let tex_y = ((d * sprite.texture.height as i32) / sprite_height) / 256;
                        if (tex_x as usize) < sprite.texture.width && (tex_y as usize) < sprite.texture.height {
                            let mut color = sprite.texture.data[sprite.texture.width * tex_y as usize + tex_x as usize];
                            if (color & 0x00FFFFFF) != 0 {
                                if let Some(&depth) = self.z_buffer.get(y as usize * self.width + column as usize) {
                                    if sprite.distance_from_camera < depth {
                        if let Some(MapCell::Empty { ceiling_texture: _, floor_texture: _, fog, fog_color }) = world.at(&camera.position.as_usize()) {
                            let fog_prop = (sprite.distance_from_camera * fog).min(1.0);
                            if fog_prop > 0.0 {
                                let mut color_bytes = color.to_le_bytes();
                                let fog_bytes = fog_color.to_le_bytes();
                                color_bytes[0] = (fog_bytes[0] as f64 * fog_prop + color_bytes[0] as f64 * (1.0 - fog_prop)) as u8;
                                color_bytes[1] = (fog_bytes[1] as f64 * fog_prop + color_bytes[1] as f64 * (1.0 - fog_prop)) as u8;
                                color_bytes[2] = (fog_bytes[2] as f64 * fog_prop + color_bytes[2] as f64 * (1.0 - fog_prop)) as u8;
                                color = u32::from_le_bytes(color_bytes);
                            }
                        }
                                        self.set_pixel(column as usize, y as usize, color);
                                        self.z_buffer[y as usize * self.width + column as usize] = sprite.distance_from_camera;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn draw_floor_and_ceiling(&mut self, camera: &Camera, world: &Map) {
        for y in 0..self.height {
            let is_floor = y > self.height / 2;
            let ray_dir_0 = &camera.direction - &camera.plane;
            let ray_dir_1 = &camera.direction + &camera.plane;

            let current_position = if is_floor { 
                (y as i32) - (self.height as i32) / 2
            } else {
                 (self.height as i32) / 2 - (y as i32)
            };
            let camera_z = if is_floor {
                0.5 * self.height as f64 + camera.height
            } else {
                0.5 * self.height as f64 - camera.height
            };
            let row_distance = camera_z / current_position as f64;

            let floor_step = row_distance * (&ray_dir_1 - &ray_dir_0) / self.width as f64;
            let mut floor = &camera.position + row_distance * &ray_dir_0;

            for x in 0..self.width {
                let cell = floor.as_usize();
                floor += &floor_step;
                match world.at(&cell) {
                    Some(MapCell::Empty { ceiling_texture, floor_texture, fog: _, fog_color: _ }) | Some(MapCell::ThinWall { texture: _, orientation: _, offset_into_cell: _, ceiling_texture, floor_texture }) => {
                    let texture_coords = Vec2 {
                        // this is also the ceiling size, but we only check the floor size
                        x: ((floor_texture.width as f64 * (floor.x - cell.x as f64)) as usize & (floor_texture.width - 1)) as i32,
                        y: ((floor_texture.height as f64 * (floor.y - cell.y as f64)) as usize & (floor_texture.height - 1)) as i32,
                    };

                    let mut color = if is_floor {
                        (floor_texture.data[(floor_texture.width as i32 * texture_coords.y + texture_coords.x) as usize] >> 1) & 8355711
                    } else {
                        (ceiling_texture.data[(ceiling_texture.width as i32 * texture_coords.y + texture_coords.x) as usize] >> 1) & 8355711
                    };
                        if let Some(MapCell::Empty { ceiling_texture: _, floor_texture: _, fog, fog_color }) = world.at(&camera.position.as_usize()) {
                            let fog_prop = ((&floor - &camera.position).length() * fog).min(1.0);
                            if fog_prop > 0.0 {
                                let mut color_bytes = color.to_le_bytes();
                                let fog_bytes = fog_color.to_le_bytes();
                                color_bytes[0] = (fog_bytes[0] as f64 * fog_prop + color_bytes[0] as f64 * (1.0 - fog_prop)) as u8;
                                color_bytes[1] = (fog_bytes[1] as f64 * fog_prop + color_bytes[1] as f64 * (1.0 - fog_prop)) as u8;
                                color_bytes[2] = (fog_bytes[2] as f64 * fog_prop + color_bytes[2] as f64 * (1.0 - fog_prop)) as u8;
                                color = u32::from_le_bytes(color_bytes);
                            }
                        }
                    self.pixels[y * self.width + x] = color;
                    },
                    Some(MapCell::Wall { texture: _ }) => {},
                    None => {},
                }
            }
        }
    }
}
