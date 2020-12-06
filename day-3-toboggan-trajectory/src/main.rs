use std::env;
use std::fs;

enum Tile {
    Empty,
    Tree,
}

struct Map {
    pub map: Vec<Vec<Tile>>,
}

impl Map {
    // Returns the number of trees hit during traversal
    fn traverse(&self, right: usize, down: usize) -> usize {
        let row_len = self.map[0].len();
        self.map
            .iter()
            .enumerate()
            // Skip rows to account for down
            .filter(|(i, _)| i % down == 0)
            .enumerate() // Re-enumerate so we can calculate right
            .map(|(j, (_, row))| match row[j * right % row_len] {
                Tile::Tree => 1,
                Tile::Empty => 0,
            })
            .fold(0, |acc, count| acc + count)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");
    let map = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Tree,
                    _ => Tile::Empty,
                })
                .collect()
        })
        .collect();

    let map = Map { map };

    let cases = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for case in cases.iter() {
        println!(
            "Trees hit with path right {} down {}: {}",
            case.0,
            case.1,
            map.traverse(case.0, case.1)
        );
    }
}
