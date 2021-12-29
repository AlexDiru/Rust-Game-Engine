use glium::backend::Facade;
use nalgebra_glm::{floor, Vec3};
use rand::{thread_rng};
use rand::{Rng};
use rand::seq::SliceRandom;
use crate::cube::Cube;

#[derive(Clone, Copy)]
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

    pub fn up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub struct Map {
    walls: Vec<Point>,
    floors: Vec<Point>,
}

impl Map {
    pub fn new() -> Map {
        // let mut walls = Vec::new();
        // walls.push(Point::new(0, 0));
        // walls.push(Point::new(0, 1));
        // walls.push(Point::new(0, 2));
        // walls.push(Point::new(0, 3));
        // walls.push(Point::new(1, 3));
        // walls.push(Point::new(2, 3));
        // walls.push(Point::new(2, 2));
        // walls.push(Point::new(2, 1));
        // walls.push(Point::new(2, 0));
        // walls.push(Point::new(1, 0));
        //
        // let mut floors = Vec::new();
        // floors.push(Point::new(1, 1));
        // floors.push(Point::new(1, 2));

        let gen = dfs_maze(21, 21);

        Map {
            floors: gen.0,
            walls: gen.1,
        }
    }

    pub fn get_walls(&self) -> &Vec<Point> {
        return &self.walls
    }

    pub fn get_floors(&self) -> &Vec<Point> {
        return &self.floors
    }
}

fn dfs_maze(x_size: i32, y_size: i32) -> (Vec<Point>, Vec<Point>) {


    let mut floors = Vec::new();
    let mut walls = Vec::new();
    for x in 0..x_size {
        for y in 0..y_size {
            walls.push(Point::new(x, y));
        }
    }

    let mut current = Point::new(1, 1);
    let mut visited = Vec::new();
    visited.push(current);

    while !visited.is_empty() {
        floors.push(current);
        walls.retain(|x| *x != current);
        current = visited.pop().unwrap();

        let mut candidates = [
            (current.up(), current.up().up()),
            (current.down(), current.down().down()),
            (current.left(), current.left().left()),
            (current.right(), current.right().right())
        ];

        candidates.shuffle(&mut thread_rng());

        for (_, candidate) in candidates.iter().enumerate() {
            let corridoor = candidate.0;
            let dest = candidate.1;
            // If in map
            if walls.contains(&dest) {
                // If not already visited
                if !floors.contains(&dest) && !visited.contains(&dest) {
                    visited.push(dest);
                    floors.push(corridoor);
                    walls.retain(|x| *x != corridoor);
                }
            }
        }
    }


    ( floors, walls )
}

fn drunken_walk(steps: i32) -> (Vec<Point>, Vec<Point>) {
    let mut floors = Vec::new();
    let mut walls : Vec<Point> = Vec::new();

    let mut current = Point::new(0, 0);

    for i in 0..steps {
        if !floors.contains(&current) {
            floors.push(current.clone());
        }

        let dir = (4.0 * rand::thread_rng().gen::<f32>()) as i32;
        match dir {
            0 => {
                current = current.up()
            }
            1 => {
                current = current.down()
            }
            2 => {
                current = current.left()
            }
            3 => {
                current = current.right()
            }
            _ => {
                println!("Unknown dir in drunken walk")
            }
        }
    }

    walls = fill_walls(&floors);

    ( floors, walls )
}

fn fill_walls(floors: &Vec<Point>) -> Vec<Point> {
    let mut walls = Vec::new();

    for (_, floor) in floors.iter().enumerate() {
        let surroundings = [ floor.up(), floor.down(), floor.left(), floor.right() ];
        for (_, sur) in surroundings.iter().enumerate() {
            if !floors.contains(sur) && !walls.contains(sur) {
                walls.push(sur.clone());
            }
        }
    }

    walls
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
            cube.set_position(wall.x as f32, 0.0f32,wall.y as f32);
            walls.push(cube);
        }

        let mut floors = Vec::new();

        for (i, floor) in map.floors.iter().enumerate() {
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