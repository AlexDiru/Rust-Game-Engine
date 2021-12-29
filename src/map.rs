use glium::backend::Facade;
use crate::cube::Cube;

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y
        }
    }
}

pub struct Map {
    walls: Vec<Point>,
    floors: Vec<Point>,
}

impl Map {
    pub fn new() -> Map {
        let mut walls = Vec::new();
        walls.push(Point::new(0, 0));
        walls.push(Point::new(0, 1));
        walls.push(Point::new(0, 2));
        walls.push(Point::new(0, 3));
        walls.push(Point::new(1, 3));
        walls.push(Point::new(2, 3));
        walls.push(Point::new(2, 2));
        walls.push(Point::new(2, 1));
        walls.push(Point::new(2, 0));
        walls.push(Point::new(1, 0));

        Map {
            walls,
            floors: Vec::new()
        }
    }

    pub fn get_walls(&self) -> &Vec<Point> {
        return &self.walls
    }
}

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

        for (i, wall) in map.walls.iter().enumerate() {
            let mut cube = Cube::new(display);
            cube.set_position(wall.x as f32, wall.y as f32, -1.0f32);
            walls.push(cube);
        }

        let mut floors = Vec::new();

        for (i, floor) in map.floors.iter().enumerate() {
            let mut cube = Cube::new(display);
            cube.set_position(floor.x as f32, floor.y as f32, -1.0f32);
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
        &self.walls
    }

    pub fn get_wall_texture(&self) -> &glium::texture::SrgbTexture2d {
        &self.wall_texture
    }

    pub fn get_floor_texture(&self) -> &glium::texture::SrgbTexture2d {
        &self.floor_texture
    }
}