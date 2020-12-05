use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);
    let mut numbers = read_input(filename);
    numbers.sort();

    for (i, x) in numbers.iter().enumerate() {
        for y in numbers.iter().skip(i + 1) {
            if x + y == 2020 {
                println!("x: {}, y: {}, result: {}", x, y, x * y);
                return;
            }
        }
    }
}

fn read_input(filename: &str) -> Vec<u32> {
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    contents
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect()
}
