use std::env;
use std::fs;

// Example line: FBFBBFFRLR
fn line_to_binary(line: &str) -> u16 {
    // Basically need to convert the first 7 characters to binary with F/L -> 0, B/R -> 1

    let len = line.len();
    let binary = line.chars().enumerate().fold(0, |acc, (i, c)| match c {
        'B' | 'R' => acc | (1 << (len - i - 1)),
        _ => acc,
    });

    // let row = binary >> 3; // Remove bottom 3 bits to get row
    // let col = binary & 0b111; // Mask bottom 3 bits to get column
    // (row, col)

    binary
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let ids: Vec<u16> = contents.lines().map(|line| line_to_binary(line)).collect();

    let (min, max): (u16, u16) = ids.iter().fold((0xffff, 0), |acc, &id| {
        (
            if id < acc.0 { id } else { acc.0 },
            if id > acc.1 { id } else { acc.1 },
        )
    });

    // Find the missing id
    let mut ids = ids;
    ids.sort();
    let item = ids
        .iter()
        .enumerate()
        .find(|(i, &id)| id - min != (*i as u16));

    println!("min: {}, max: {}", min, max);
    println!("missing row id in range: {}", item.unwrap().1 - 1);
}
