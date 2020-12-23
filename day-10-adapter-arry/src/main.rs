use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut outlets: Vec<u32> = contents
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    outlets.sort();
    outlets.insert(0, 0);
    outlets.push(outlets.last().unwrap() + 3);

    let mut jolt_count = (0, 0);
    for i in 0..outlets.len() - 1 {
        let j = i + 1;

        let diff = (outlets[j] - outlets[i]) as u64;

        // Tally
        match diff {
            1 => jolt_count.0 += 1,
            2 => (),
            3 => jolt_count.1 += 1,
            _ => (),
        }
    }

    // Permutations
    // For each adapter, the number of paths to the end are the sum of the paths to the end for each
    // adapter that can be connected next. Calculate these start at the end and work our way backwards.
    // This could be done better. Don't need to maintain the whole list, just the tail 3
    let mut paths_to_end: Vec<u64> = vec![0; outlets.len()];
    paths_to_end[outlets.len() - 1] = 1;
    for (i, jolts) in outlets.iter().enumerate().rev().skip(1) {
        let mut paths = 0;
        for j in i + 1..=i + 3 {
            match outlets.get(j) {
                Some(adapter_jolts) => {
                    if *adapter_jolts <= jolts + 3 {
                        paths += paths_to_end[j];
                    }
                }
                None => (),
            }
        }
        paths_to_end[i] = paths;
    }

    // println!("outlets: {:?}", outlets);
    // println!("paths to end: {:?}", paths_to_end);

    println!(
        "{} with 1 jolt diff\r\n{} with 3 jolt diff\r\n{} multiplied",
        jolt_count.0,
        jolt_count.1,
        jolt_count.0 * jolt_count.1
    );
    println!("{} permutations", paths_to_end[0]);
}

/*
5, 6, 7, 10, 11, 12,
5, 6, 7, 10, 12,
5, 7, 10, 11, 12,
5, 7, 10, 12,
6, 7, 10, 11, 12,
6, 7, 10, 12,
7, 10, 11, 12,
7, 10, 12,
*/
