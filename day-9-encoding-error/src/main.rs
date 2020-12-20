use std::cmp::Eq;
use std::env;
use std::fs;
use std::ops::Add;

struct CyclicBuffer<T> {
    buffer: Vec<T>,
    write_head: usize,
}

impl<T> CyclicBuffer<T>
where
    T: Add<Output = T>,
    T: Copy,
    T: Eq,
    T: PartialEq,
{
    pub fn from(buffer: Vec<T>) -> CyclicBuffer<T> {
        CyclicBuffer {
            write_head: buffer.len() - 1,
            buffer,
        }
    }

    pub fn push(&mut self, item: T) {
        self.write_head = (self.write_head + 1) % self.buffer.len();
        self.buffer[self.write_head] = item;
    }

    pub fn is_binary_sum(&self, item: T) -> bool {
        for i in 0..self.buffer.len() {
            for j in 0..self.buffer.len() {
                if item == self.buffer[i] + self.buffer[j] {
                    return true;
                }
            }
        }
        false
    }
}

fn part_1(contents: &str, preamble_len: usize) -> u64 {
    let buffer: Vec<u64> = contents
        .lines()
        .take(preamble_len)
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let mut cyclic_buffer = CyclicBuffer::from(buffer);

    contents
        .lines()
        .skip(preamble_len)
        .map(|line| line.parse::<u64>().unwrap())
        .find(|&val| {
            if cyclic_buffer.is_binary_sum(val) {
                cyclic_buffer.push(val);
                false
            } else {
                true
            }
        })
        .unwrap()
}

fn part_2(contents: &str, preamble_len: usize, invalid_num: u64) -> (u64, u64) {
    let buffer: Vec<u64> = contents
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let n = buffer.len();
    for i in 0..n {
        let mut j = i + 1;
        while j < n {
            let sum = buffer[i..=j].iter().fold(0, |a, b| a + b);

            if sum == invalid_num {
                // Return the min/max
                return buffer[i..=j]
                    .iter()
                    .fold((buffer[i], buffer[i]), |extrema, &val| {
                        (
                            if val < extrema.0 { val } else { extrema.0 },
                            if val > extrema.1 { val } else { extrema.1 },
                        )
                    });
            } else if sum > invalid_num {
                break;
            }

            j += 1;
        }
    }

    (0, 0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let preamble_len = args[2]
        .parse::<usize>()
        .expect("Need preamble length as argument");

    println!("Reading input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let invalid_num = part_1(&contents, preamble_len);

    println!("part 1: {}", invalid_num);

    let range = part_2(&contents, preamble_len, invalid_num);
    println!("part 2: {:?} {}", range, range.0 + range.1);
}
