use std::env;
use std::fs;

fn part_1(buses: &Vec<(u64, u64)>, target_time: u64) -> u64 {
    let (wait_time, bus) = buses.iter().fold((u64::MAX, u64::MAX), |acc, &(_, bus)| {
        let wait_time = bus * (target_time / bus + 1) - target_time;
        if wait_time < acc.0 {
            (wait_time, bus)
        } else {
            acc
        }
    });

    wait_time * bus
}

fn extended_gcd(mut a: i64, mut b: i64) -> (i64, i64) {
    let (mut x, mut y, mut u, mut v) = (0, 1, 1, 0);

    while a != 0 {
        let (q, r) = (b / a, b % a);
        let (m, n) = (x - u * q, y - v * q);

        // After rust allows assigning in destructuring we shall be able to write
        // (b,a,x,y,u,v) = (a,r,u,v,m,n)
        b = a;
        a = r;
        x = u;
        y = v;
        u = m;
        v = n;
    }

    (x, y)
}

fn part_2(buses: &Vec<(u64, u64)>) -> i64 {
    // Apply chinese remainder theorem
    // https://rosettacode.org/wiki/Chinese_remainder_theorem
    // All input bus ids are coprime

    let n: i64 = buses.iter().map(|&(_, id)| id as i64).product();

    let x: i64 = buses
        .iter()
        .map(|&(index, id)| {
            let ai = index as i64;
            let n_over_ni = n / id as i64;
            let (_, si) = extended_gcd(id as i64, n_over_ni);

            ai * si * n_over_ni
        })
        .sum();

    x.abs() % n
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut lines = contents.lines();

    let target = lines.next().unwrap().parse::<u64>().unwrap();

    // bus position, bus id
    let buses: Vec<(u64, u64)> = lines
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|&(_, s)| s != "x")
        .map(|(i, s)| (i as u64, s.parse::<u64>().unwrap()))
        .collect();

    println!("part 1: {}", part_1(&buses, target));
    println!("part 2: {}", part_2(&buses));
}
