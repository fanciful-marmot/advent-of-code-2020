use std::env;
use std::fs;

use regex::Regex;

fn validate_year(year: &str, min: u32, max: u32, regex: &Regex) -> bool {
    let cap = regex.captures(year);
    match cap {
        Some(caps) => {
            let year = caps[1].parse::<u32>().unwrap();
            if year >= min && year <= max {
                true
            } else {
                false
            }
        }
        None => false,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);

    let year_regex = Regex::new(r"^(\d{4})$").unwrap();
    let height_regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let hair_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let eye_regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let pid_regex = Regex::new(r"^(\d{9})$").unwrap();

    let contents = fs::read_to_string(filename).expect("Failed to read file");
    let valid_passports = contents
        .split("\r\n\r\n")
        .map(|chunk| {
            let bit_field = chunk
                // Split into key:val strings
                .split_ascii_whitespace()
                // Extract (key, val)
                .map(|key_val| {
                    let mut iter = key_val.split(':');
                    (iter.next().unwrap(), iter.next().unwrap())
                })
                // Use bitfield to track valid entries
                .fold(0, |acc, (key, val)| {
                    match key {
                        "byr" => {
                            if validate_year(val, 1920, 2002, &year_regex) {
                                acc | 1
                            } else {
                                acc
                            }
                        }
                        "iyr" => {
                            if validate_year(val, 2010, 2020, &year_regex) {
                                acc | 2
                            } else {
                                acc
                            }
                        }
                        "eyr" => {
                            if validate_year(val, 2020, 2030, &year_regex) {
                                acc | 4
                            } else {
                                acc
                            }
                        }
                        "hgt" => {
                            let cap = height_regex.captures(val);
                            match cap {
                                Some(caps) => {
                                    if caps.len() == 3 {
                                        let range = if &caps[2] == "in" {
                                            (59, 76)
                                        } else {
                                            (150, 193)
                                        };
                                        let h = caps[1].parse::<u32>().unwrap();
                                        if h >= range.0 && h <= range.1 {
                                            acc | 8
                                        } else {
                                            acc
                                        }
                                    } else {
                                        acc
                                    }
                                }
                                None => acc,
                            }
                        }
                        "hcl" => {
                            if hair_regex.is_match(val) {
                                acc | 16
                            } else {
                                acc
                            }
                        }
                        "ecl" => {
                            if eye_regex.is_match(val) {
                                acc | 32
                            } else {
                                acc
                            }
                        }
                        "pid" => {
                            if pid_regex.is_match(val) {
                                acc | 64
                            } else {
                                acc
                            }
                        }
                        // "cid" => acc | 128, // Skip country for now
                        _ => acc,
                    }
                });
            if bit_field >= 128 - 1 {
                1 // valid
            } else {
                0 // invalid
            }
        })
        .fold(0, |acc, v| acc + v);

    println!("Number of valid passports: {}", valid_passports);
}
