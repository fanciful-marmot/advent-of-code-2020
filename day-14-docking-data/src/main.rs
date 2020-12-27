use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(PartialEq, Eq, Copy, Clone)]
enum MaskBit {
    ZERO,
    ONE,
    UNSET,
}

// Sets a specific bit in a number
fn set_bit(num: u64, index: u8, bit_set: bool) -> u64 {
    if bit_set {
        num | (1 << index) // Set corresponding bit to 1
    } else {
        num & (!0 - (1 << index)) // Sets corresponding bit to 0
    }
}

// Apply a mask to a number
fn apply(mask: &Vec<MaskBit>, num: u64) -> u64 {
    mask.iter()
        .enumerate()
        .fold(num, |acc, (i, bit)| match bit {
            MaskBit::UNSET => acc,
            _ => set_bit(acc, i as u8, *bit == MaskBit::ONE),
        })
}

// fn print_mask(mask: &Vec<MaskBit>) -> () {
//     print!("mask:   0b");
//     mask.iter().for_each(|bit| match bit {
//         MaskBit::ZERO => print!("0"),
//         MaskBit::ONE => print!("1"),
//         MaskBit::UNSET => print!("X"),
//     });
//     println!();
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = vec![MaskBit::UNSET; 36];
    contents.lines().for_each(|line| {
        let mut split = line.split(" ");
        let command = split.next().unwrap();
        split.next(); // Skip "="
        let value = split.next().unwrap();

        match command {
            "mask" => value.chars().rev().enumerate().for_each(|(i, c)| {
                mask[i] = match c {
                    '0' => MaskBit::ZERO,
                    '1' => MaskBit::ONE,
                    _ => MaskBit::UNSET,
                };
            }),
            c => {
                let address = c[4..command.len() - 1].parse::<u64>().unwrap();
                let num = value.parse::<u64>().unwrap();
                let result = apply(&mask, num);
                mem.insert(address, result);
            }
        }
    });

    let sum: u64 = mem.values().sum();

    println!("sum of memory: {}", sum);
}
