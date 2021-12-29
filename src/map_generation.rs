use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use crate::Map;
use crate::point::Point;

pub fn dfs_maze(x_size: i32, y_size: i32) -> Map {
    let mut floors = Vec::new();
    let mut walls = Vec::new();
    for x in 0..x_size {
        for y in 0..y_size {
            walls.push(Point::new(x, y));
        }
    }

    let mut current = Point::new(1, 1);
    let start = current;
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

    Map::new(walls, floors, start)
}

pub fn drunken_walk(steps: i32) -> Map {
    let mut floors = Vec::new();
    let mut walls : Vec<Point> = Vec::new();

    let mut current = Point::new(0, 0);
    let start = current;

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

    Map::new(walls, floors, start)
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