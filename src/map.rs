use crate::map_generation::dfs_maze;
use crate::point::Point;

pub struct Map {
    walls: Vec<Point>,
    floors: Vec<Point>,
}

impl Map {
    pub fn new() -> Map {
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