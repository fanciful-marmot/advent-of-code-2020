use std::env;
use std::fs;
use std::mem::swap;

#[derive(PartialEq, Copy, Clone)]
enum Cell {
    EMPTY,
    OCCUPIED,
    FLOOR,
}

struct Plane {
    seats: Vec<Vec<Cell>>,
    swap_seats: Vec<Vec<Cell>>,
}

impl Plane {
    pub fn from_input(input: &String) -> Plane {
        let seats: Vec<Vec<Cell>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'L' => Cell::EMPTY,
                        _ => Cell::FLOOR,
                    })
                    .collect()
            })
            .collect();

        let swap_seats: Vec<Vec<Cell>> = seats.iter().map(|row| row.clone()).collect();

        Plane { seats, swap_seats }
    }

    pub fn num_occupied_seats(&self) -> u64 {
        self.seats.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |acc, col| {
                acc + if *col == Cell::OCCUPIED { 1 } else { 0 }
            })
        })
    }

    fn count_adjacent_occupied(&self, i: usize, j: usize) -> u32 {
        let row = &self.seats[i];

        let mut occupied = 0;
        let r_range = (
            if i > 0 { i - 1 } else { i },
            if i < self.seats.len() - 1 { i + 1 } else { i },
        );
        let c_range = (
            if j > 0 { j - 1 } else { j },
            if j < row.len() - 1 { j + 1 } else { j },
        );
        for r in r_range.0..=r_range.1 {
            for c in c_range.0..=c_range.1 {
                if (r != i || c != j) && self.seats[r][c] == Cell::OCCUPIED {
                    occupied += 1;
                }
            }
        }

        occupied
    }

    fn is_occupied_in_direction(&self, i: usize, j: usize, rise: i64, run: i64) -> bool {
        if rise == 0 && run == 0 {
            return false;
        }

        let mut r = (i as i64) + run;
        let mut c = (j as i64) + rise;
        while let Some(cell) = self
            .seats
            .get(r as usize)
            .and_then(|row| row.get(c as usize))
        {
            match *cell {
                Cell::EMPTY => return false,
                Cell::OCCUPIED => return true,
                Cell::FLOOR => (),
            }

            r += run;
            c += rise;
        }

        false
    }

    fn count_visible_occupied(&self, i: usize, j: usize) -> u32 {
        let mut occupied = 0;
        for rise in -1..=1 {
            for run in -1..=1 {
                if self.is_occupied_in_direction(i, j, rise, run) {
                    occupied += 1;
                }
            }
        }

        occupied
    }

    pub fn tick(&mut self) -> bool {
        // Track if anything changed
        let mut change = false;

        // Write next tick into new_seats
        for (i, row) in self.seats.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == Cell::FLOOR {
                    continue;
                }

                // Part 1
                // let occupied = self.count_adjacent_occupied(i, j);

                // // Update seat
                // self.swap_seats[i][j] = match (cell, occupied) {
                //     (Cell::EMPTY, 0) => {
                //         change = true;
                //         Cell::OCCUPIED
                //     }
                //     (Cell::OCCUPIED, n) if n >= 4 => {
                //         change = true;
                //         Cell::EMPTY
                //     }
                //     _ => *cell,
                // }

                // Part 2
                let occupied = self.count_visible_occupied(i, j);

                // Update seat
                self.swap_seats[i][j] = match (cell, occupied) {
                    (Cell::EMPTY, 0) => {
                        change = true;
                        Cell::OCCUPIED
                    }
                    (Cell::OCCUPIED, n) if n >= 5 => {
                        change = true;
                        Cell::EMPTY
                    }
                    _ => *cell,
                }
            }
        }

        // Swap the buffers
        swap(&mut self.seats, &mut self.swap_seats);

        change
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut plane = Plane::from_input(&contents);

    while plane.tick() {}

    println!("Occupied seats: {}", plane.num_occupied_seats());
}
