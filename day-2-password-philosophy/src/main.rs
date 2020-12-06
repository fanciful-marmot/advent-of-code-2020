use std::env;
use std::fs;

struct Password<'a> {
    pub range: (u8, u8),
    pub letter: char,
    pub password: &'a str,
}

impl Password<'_> {
    fn is_valid_part_1(&self) -> bool {
        let count = self.password.chars().filter(|c| c == &self.letter).count();

        count >= (self.range.0 as usize) && count <= (self.range.1 as usize)
    }

    fn is_valid_part_2(&self) -> bool {
        let bytes = self.password.as_bytes();

        let first_match = bytes[(self.range.0 - 1) as usize] as char == self.letter;
        let second_match = bytes[(self.range.1 - 1) as usize] as char == self.letter;

        match (first_match, second_match) {
            (true, false) => true,
            (false, true) => true,
            (_, _) => false,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");

    // Part 1
    let count = contents
        .lines()
        .map(|l| {
            let components: Vec<&str> = l.split_ascii_whitespace().collect();
            let range: Vec<&str> = components[0].split('-').collect();
            Password {
                range: (
                    range[0].parse::<u8>().unwrap(),
                    range[1].parse::<u8>().unwrap(),
                ),
                letter: components[1].chars().nth(0).unwrap(),
                password: components.last().unwrap(),
            }
        })
        // .filter(|password| password.is_valid_part_1())
        .filter(|password| password.is_valid_part_2())
        .count();

    println!("Valid passwords: {}", count);
}
