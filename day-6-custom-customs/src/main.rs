use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let sum = contents.split("\r\n\r\n").fold(0, |acc, group| {
        let mut group_flags = group.lines().enumerate().fold(0, |acc, (i, line)| {
            let line_flags: u32 = line
                .chars()
                .fold(0, |acc, c| acc | (1 << (c as u8 - 'a' as u8)));

            // part 1, flag any question at least one person said yes to
            // acc | line_flags

            // part2, flag any question everyone in the group said yes to
            if i == 0 {
                // If we're the first line, set the mask
                line_flags
            } else {
                // Only include answers that others in the group have said yes to
                acc & line_flags
            }
        });

        let mut count = 0;
        while group_flags > 0 {
            count += group_flags & 1;
            group_flags = group_flags >> 1;
        }

        acc + count
    });

    println!("customs sum: {}", sum);
}
