mod camera;
mod framebuffer;
mod map;
mod texture;
mod util;
mod vec2;

use minifb::{Window, WindowOptions};

use camera::{Camera, Intersection};
use framebuffer::Framebuffer;
use map::{Map, MapCell};
use texture::{Font, Texture};
use util::{Orientation, Side, Sprite, Step};
use vec2::Vec2;

use std::rc::Rc;
use std::time::Instant;

fn main() {
    let mut framebuffer = Framebuffer::new(600, 800);
    let mut camera = Camera {
        position: Vec2 { x: 3.0, y: 12.0 },
        direction: Vec2 { x: -1.0, y: 0.0 },
        plane: Vec2 {
            x: 0.0,
            y: (framebuffer.width as f64 / framebuffer.height as f64) / 2.0,
        },
        height: 0.0,
    };

    let mut time = Instant::now();
    let mut old_time: Instant;

    let font = Font::load_from_bmp(&include_bytes!("../res/font.bmp").to_vec(), 8);
    let textures: Vec<Rc<Texture>> = vec![
        Rc::new(Texture::load_from_bmp(
            &include_bytes!("../res/textures/eagle.bmp").to_vec(),
        )),
        Rc::new(Texture::load_from_bmp(
            &include_bytes!("../res/textures/redbrick.bmp").to_vec(),
        )),
        Rc::new(Texture::load_from_bmp(
            &include_bytes!("../res/textures/purplestone.bmp").to_vec(),
        )),
        Rc::new(Texture::load_from_bmp(
            &include_bytes!("../res/textures/greystone.bmp").to_vec(),
        )),
        Rc::new(Texture::load_from_bmp(
            &include_bytes!("../res/textures/bluestone.bmp").to_vec(),
        )),
        Rc::new(Texture::load_from_bmp(
            &include_bytes!("../res/textures/mossy.bmp").to_vec(),
        )),
        Rc::new(Texture::load_from_bmp(
            &include_bytes!("../res/textures/wood.bmp").to_vec(),
        )),
        Rc::new(Texture::load_from_bmp(
            &include_bytes!("../res/textures/colorstone.bmp").to_vec(),
        )),
        Rc::new(Texture::load_from_bmp(
            &include_bytes!("../res/textures/barrel.bmp").to_vec(),
        )),
        Rc::new(Texture::load_from_bmp(
            &include_bytes!("../res/textures/pillar.bmp").to_vec(),
        )),
        Rc::new(Texture::load_from_bmp(
            &include_bytes!("../res/textures/greenlight.bmp").to_vec(),
        )),
    ];

    let world = Map::new(&textures);

    let mut sprites = vec![
        Sprite {
            position: Vec2 { x: 3.0, y: 8.0 },
            texture: textures[8].clone(),
            scale_factor: Vec2 { x: 1.0, y: 1.0 },
            vertical_offset: 0.0,
            distance_from_camera: 0.0,
        },
        Sprite {
            position: Vec2 { x: 3.0, y: 6.0 },
            texture: textures[8].clone(),
            scale_factor: Vec2 { x: 1.5, y: 1.5 },
            vertical_offset: -150.0,
            distance_from_camera: 0.0,
        },
    ];

    let mut window = Window::new(
        "Raycasting Demo",
        framebuffer.width,
        framebuffer.height,
        WindowOptions::default(),
    )
    .unwrap();
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() {
        framebuffer.draw_floor_and_ceiling(&camera, &world);
        for x in 0..framebuffer.width {
            let mut side_dist = Vec2::<f64>::new();
            let mut ray = camera.get_ray(x, framebuffer.width);
            let mut map = camera.position.as_usize();
            let delta_dist = Vec2 {
                x: (1.0 / ray.direction.x).abs(),
                y: (1.0 / ray.direction.y).abs(),
            };
            let mut side: Side;
            let step = Vec2 {
                x: Step::from(ray.direction.x < 0.0),
                y: Step::from(ray.direction.y < 0.0),
            };
            if ray.direction.x < 0.0 {
                side_dist.x = (camera.position.x - (map.x as f64)) * delta_dist.x;
            } else {
                side_dist.x = ((map.x as f64) + 1.0 - camera.position.x) * delta_dist.x;
            }
            if ray.direction.y < 0.0 {
                side_dist.y = (camera.position.y - (map.y as f64)) * delta_dist.y;
            } else {
                side_dist.y = ((map.y as f64) + 1.0 - camera.position.y) * delta_dist.y;
            }

            loop {
                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist.x;
                    match step.x {
                        Step::Left => map.x -= 1,
                        Step::Right => map.x += 1,
                    }
                    side = Side::X;
                } else {
                    side_dist.y += delta_dist.y;
                    match step.y {
                        Step::Left => map.y -= 1,
                        Step::Right => map.y += 1,
                    }
                    side = Side::Y;
                }
                match world.at(&map) {
                    Some(MapCell::Wall { texture }) => {
                        ray.intersections.push(Intersection {
                            side: side.clone(),
                            step: step.clone(),
                            map_coordinates: map.clone(),
                            wall_offset: Vec2 { x: 0.0, y: 0.0 },
                        });
                        if !texture.has_transparency {
                            break;
                        }
                    }
                    Some(MapCell::ThinWall {
                        texture,
                        orientation,
                        offset_into_cell,
                        ceiling_texture: _,
                        floor_texture: _,
                    }) => match orientation {
                        Orientation::XAxis => {
                            if side_dist.x - (delta_dist.x / (1.0 / offset_into_cell)) > side_dist.y
                            {
                                continue;
                            } else {
                                ray.intersections.push(Intersection {
                                    side: Side::X,
                                    step: step.clone(),
                                    map_coordinates: map.clone(),
                                    wall_offset: Vec2 {
                                        x: offset_into_cell * step.x.value() as f64,
                                        y: 0.0,
                                    },
                                });
                                if !texture.has_transparency {
                                    break;
                                }
                            }
                        }
                        Orientation::YAxis => {
                            if side_dist.y - (delta_dist.y / (1.0 / offset_into_cell)) > side_dist.x
                            {
                                continue;
                            } else {
                                ray.intersections.push(Intersection {
                                    side: Side::Y,
                                    step: step.clone(),
                                    map_coordinates: map.clone(),
                                    wall_offset: Vec2 {
                                        x: 0.0,
                                        y: offset_into_cell * step.y.value() as f64,
                                    },
                                });
                                if !texture.has_transparency {
                                    break;
                                }
                            }
                        }
                    },
                    Some(MapCell::Empty {
                        ceiling_texture: _,
                        floor_texture: _,
                        fog: _,
                        fog_color: _,
                    }) => continue,
                    None => break,
                }
            }
            for intersection in &ray.intersections {
                let perp_wall_dist = match &intersection.side {
                    Side::X => {
                        ((intersection.map_coordinates.x as f64) - camera.position.x
                            + intersection.wall_offset.x
                            + ((1 - intersection.step.x.value()) as f64) / 2.0)
                            / ray.direction.x
                    }
                    Side::Y => {
                        ((intersection.map_coordinates.y as f64) - camera.position.y
                            + intersection.wall_offset.y
                            + ((1 - intersection.step.y.value()) as f64) / 2.0)
                            / ray.direction.y
                    }
                };
                if let Some(cell) = world.at(&intersection.map_coordinates) {
                    framebuffer.draw_wall(
                        &camera,
                        x,
                        perp_wall_dist,
                        cell,
                        &intersection.side,
                        &ray,
                        &world,
                    );
                }
            }
        }
        framebuffer.draw_sprites(&camera, &mut sprites, &world);
        old_time = time;
        time = Instant::now();
        let frame_time = (time - old_time).as_secs_f64();
        framebuffer.write_ascii_string(
            0,
            0,
            &format!("{:.3}", (1.0 / frame_time)).into_bytes(),
            &font,
            0x00FFFFFF,
        );
        framebuffer.write_ascii_string(
            0,
            font.glyph_size,
            &format!("x: {:.3}, y: {:.3}", camera.position.x, camera.position.y).into_bytes(),
            &font,
            0x00FFFFFF,
        );
        camera.update_position_with_keys(frame_time, &window, &world);
        window
            .update_with_buffer(&framebuffer.pixels, framebuffer.width, framebuffer.height)
            .unwrap();
        framebuffer.clear_z_buffer();
    }
}
