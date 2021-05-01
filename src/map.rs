use std::rc::Rc;

use crate::vec2::Vec2;
use crate::util::Orientation;
use crate::texture::Texture;

pub enum MapCell {
    Empty { 
        ceiling_texture: Rc<Texture>,
        floor_texture: Rc<Texture>,
        fog: f64,
        fog_color: u32,
    },
    Wall {
        texture: Rc<Texture>,
    },
    ThinWall {
        texture: Rc<Texture>,
        orientation: Orientation,
        offset_into_cell: f64,
        ceiling_texture: Rc<Texture>,
        floor_texture: Rc<Texture>,
    },
}

pub struct Map {
    width: usize,
    height: usize,
    cells: Vec<MapCell>,
}

impl Map {
    pub fn new(texture_atlas: &Vec<Rc<Texture>>) -> Map {
        let layout = include_str!("../res/map.txt").replace("\n", "");
        let mut cells = Vec::with_capacity(layout.len());
        for cell in layout.split(',') {
            match cell {
                "0" => cells.push(MapCell::Empty { ceiling_texture: texture_atlas[6].clone(), floor_texture: texture_atlas[3].clone(), fog: 0.08, fog_color: 0x00000000 }),
                _ => cells.push(MapCell::Wall { texture: texture_atlas[cell.parse::<usize>().unwrap() - 1].clone() }),
            }
        }
        cells[5 * 24 + 9] = MapCell::ThinWall { texture: texture_atlas[6].clone(), orientation: Orientation::XAxis, offset_into_cell: 0.5, ceiling_texture: texture_atlas[6].clone(), floor_texture: texture_atlas[3].clone() };
        cells[5 * 24 + 10] = MapCell::ThinWall { texture: texture_atlas[5].clone(), orientation: Orientation::XAxis, offset_into_cell: 0.5, ceiling_texture: texture_atlas[6].clone(), floor_texture: texture_atlas[3].clone() };
        cells[5 * 24 + 11] = MapCell::ThinWall { texture: texture_atlas[4].clone(), orientation: Orientation::XAxis, offset_into_cell: 0.5, ceiling_texture: texture_atlas[6].clone(), floor_texture: texture_atlas[3].clone() };
        cells[3 * 24 + 2] = MapCell::ThinWall { texture: texture_atlas[9].clone(), orientation: Orientation::YAxis, offset_into_cell: 0.5, ceiling_texture: texture_atlas[6].clone(), floor_texture: texture_atlas[3].clone() };
        Map {
            width: 24,
            height: 24,
            cells,
        }
    }

    pub fn at(&self, position: &Vec2<usize>) -> Option<&MapCell> {
        if position.x < self.height && position.y < self.width {
            self.cells.get(position.x * self.width + position.y)
        } else {
            None
        }
    }
}
