use glium::backend::Facade;
use crate::{Cube, Map};

pub struct RenderableMap {
    map: Map,
    walls: Vec<Cube>,
    floors: Vec<Cube>,
    floor_texture: glium::texture::SrgbTexture2d,
    wall_texture: glium::texture::SrgbTexture2d,
}

impl RenderableMap {
    pub fn new<F: ?Sized>(map: Map, display: &F, floor_texture: glium::texture::SrgbTexture2d, wall_texture: glium::texture::SrgbTexture2d) -> RenderableMap where F: Facade {
        let mut walls = Vec::new();

        for (i, wall) in map.get_walls().iter().enumerate() {
            let mut cube = Cube::new(display);
            cube.set_position(wall.x as f32, 0.0f32,wall.y as f32);
            walls.push(cube);
        }

        let mut floors = Vec::new();

        for (i, floor) in map.get_floors().iter().enumerate() {
            let mut cube = Cube::new(display);
            cube.set_position(floor.x as f32, -1.0f32, floor.y as f32);
            floors.push(cube);
        }

        RenderableMap {
            map,
            floors,
            walls,
            floor_texture,
            wall_texture
        }
    }

    pub fn get_walls(&self) -> &Vec<Cube> {
        &self.walls
    }

    pub fn get_floors(&self) -> &Vec<Cube> {
        &self.floors
    }

    pub fn get_wall_texture(&self) -> &glium::texture::SrgbTexture2d {
        &self.wall_texture
    }

    pub fn get_floor_texture(&self) -> &glium::texture::SrgbTexture2d {
        &self.floor_texture
    }
}