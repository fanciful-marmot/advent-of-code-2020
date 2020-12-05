use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);
    let mut numbers = read_input(filename);
    numbers.sort();

    println!("part 1: {}", part1(&numbers).unwrap());
    println!("part 2: {}", part2(&numbers).unwrap());
}

fn read_input(filename: &str) -> Vec<u32> {
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    contents
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect()
}

// Finds 2 numbers in the sorted input list that sum to 2020 and multiplies them together
fn part1(values: &Vec<u32>) -> Option<u32> {
    for (i, x) in values.iter().enumerate() {
        for y in values.iter().skip(i + 1) {
            if x + y == 2020 {
                // println!("x: {}, y: {}, result: {}", x, y, x * y);
                return Some(x * y);
            }
        }
    }

    return None;
}

fn part2(values: &Vec<u32>) -> Option<u32> {
    for (i, x) in values.iter().enumerate() {
        for (j, y) in values.iter().enumerate().skip(i + 1) {
            for z in values.iter().skip(j + 1) {
                if x + y + z == 2020 {
                    // println!("x: {}, y: {}, result: {}", x, y, x * y);
                    return Some(x * y * z);
                }
            }
        }
    }

    return None;
}
