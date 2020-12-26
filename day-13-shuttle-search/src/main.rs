use std::env;
use std::fs;

fn part_1(buses: &Vec<u32>, target_time: u32) -> u32 {
    let (wait_time, bus) = buses.iter().fold((u32::MAX, u32::MAX), |acc, &bus| {
        let wait_time = bus * (target_time / bus + 1) - target_time;
        if wait_time < acc.0 {
            (wait_time, bus)
        } else {
            acc
        }
    });

    wait_time * bus
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut lines = contents.lines();

    let target = lines.next().unwrap().parse::<u32>().unwrap();

    let buses: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|&s| s != "x")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    println!("part 1: {}", part_1(&buses, target));
}
