use crate::map_generation::{dfs_maze, drunken_walk};
use crate::point::Point;

pub struct Map {
    walls: Vec<Point>,
    floors: Vec<Point>,
    start: Point,
}

impl Map {
    pub fn new(walls: Vec<Point>, floors: Vec<Point>, start: Point) -> Map {
        Map { floors, walls, start }
    }

    pub fn dfs_maze() -> Map {
        dfs_maze(21, 21)
    }

    pub fn drunken_walk() -> Map {
        drunken_walk(400)
    }

    pub fn get_walls(&self) -> &Vec<Point> {
        &self.walls
    }

    pub fn get_floors(&self) -> &Vec<Point> {
        &self.floors
    }

    pub fn get_start(&self) -> &Point {
        &self.start
    }
}