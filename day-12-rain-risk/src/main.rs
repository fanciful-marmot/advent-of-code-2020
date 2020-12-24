use std::env;
use std::fs;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    // Clockwise iteration through directions
    fn next(&self) -> Direction {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum RelativeDirection {
    CW,
    CCW,
}

struct Waypoint {
    position: (i32, i32),
}

impl Waypoint {
    fn new() -> Waypoint {
        Waypoint { position: (10, 1) }
    }

    fn absolute_move(&mut self, direction: Direction, distance: i32) -> () {
        match direction {
            Direction::NORTH => self.position.1 += distance,
            Direction::SOUTH => self.position.1 -= distance,
            Direction::EAST => self.position.0 += distance,
            Direction::WEST => self.position.0 -= distance,
        }
    }

    fn turn(&mut self, relative_direction: RelativeDirection, degrees: u32) -> () {
        let ticks = if relative_direction == RelativeDirection::CW {
            (degrees / 90) as i32
        } else {
            -((degrees / 90) as i32) + 4
        };

        // Rotate 90 clockwise the number of ticks
        for _ in 0..ticks {
            self.position = (self.position.1, -self.position.0);
        }
    }
}

struct Ship {
    heading: Direction,
    position: (i32, i32),
}

impl Ship {
    fn new() -> Ship {
        Ship {
            heading: Direction::EAST,
            position: (0, 0),
        }
    }

    fn absolute_move(&mut self, direction: Direction, distance: i32) -> () {
        match direction {
            Direction::NORTH => self.position.1 += distance,
            Direction::SOUTH => self.position.1 -= distance,
            Direction::EAST => self.position.0 += distance,
            Direction::WEST => self.position.0 -= distance,
        }
    }

    fn relative_move(&mut self, distance: i32) -> () {
        self.absolute_move(self.heading, distance)
    }

    fn waypoint_move(&mut self, waypoint: &Waypoint, times: i32) -> () {
        self.position = (
            self.position.0 + waypoint.position.0 * times,
            self.position.1 + waypoint.position.1 * times,
        );
    }

    fn turn(&mut self, relative_direction: RelativeDirection, degrees: u32) -> () {
        let ticks = if relative_direction == RelativeDirection::CW {
            (degrees / 90) as i32
        } else {
            -((degrees / 90) as i32) + 4
        };

        let mut new_heading = self.heading;
        for _ in 0..ticks {
            new_heading = new_heading.next();
        }

        self.heading = new_heading;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut ship = Ship::new();
    let mut waypoint = Waypoint::new();

    contents.lines().for_each(|line| {
        let command_letter = &line[0..1];
        let value = line[1..].parse::<i32>().unwrap();

        // Part 1
        // match command_letter {
        //     "N" => ship.absolute_move(Direction::NORTH, value),
        //     "S" => ship.absolute_move(Direction::SOUTH, value),
        //     "E" => ship.absolute_move(Direction::EAST, value),
        //     "W" => ship.absolute_move(Direction::WEST, value),
        //     "L" => ship.turn(RelativeDirection::CCW, value as u32),
        //     "R" => ship.turn(RelativeDirection::CW, value as u32),
        //     "F" => ship.relative_move(value),
        //     _ => {
        //         eprintln!("Unexpected command: {}", command_letter);
        //         unreachable!();
        //     }
        // }

        // Part 2
        match command_letter {
            "N" => waypoint.absolute_move(Direction::NORTH, value),
            "S" => waypoint.absolute_move(Direction::SOUTH, value),
            "E" => waypoint.absolute_move(Direction::EAST, value),
            "W" => waypoint.absolute_move(Direction::WEST, value),
            "L" => waypoint.turn(RelativeDirection::CCW, value as u32),
            "R" => waypoint.turn(RelativeDirection::CW, value as u32),
            "F" => ship.waypoint_move(&waypoint, value),
            _ => {
                eprintln!("Unexpected command: {}", command_letter);
                unreachable!();
            }
        }
    });

    println!(
        "Waypoint is at ({}, {})",
        waypoint.position.0, waypoint.position.1
    );
    println!("Ship is at ({}, {})", ship.position.0, ship.position.1);

    println!(
        "Manhattan distance: {}",
        ship.position.0.abs() + ship.position.1.abs()
    );
}
