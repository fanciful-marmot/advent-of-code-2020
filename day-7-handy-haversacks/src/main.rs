use std::collections::HashMap;
use std::env;
use std::fs;

use regex::Regex;

struct Bag {
    pub id: String,
    pub contents: Vec<(String, usize)>, // bag id, count

    // Used to track if a bag eventually contains the shiny bag (used for caching computation)
    pub eventually_contains_shiny: Option<bool>,
}

fn contains_shiny<'a>(bags: &mut HashMap<String, Bag>, id: &str) -> bool {
    let bag = bags.get_mut(id).unwrap();
    let contents = bag.contents.clone(); // Have to clone to satisfy borrow checker
    let eventually_contains_shiny = bag.eventually_contains_shiny;
    let success = match eventually_contains_shiny {
        Some(contains) => contains,
        None => contents
            .iter()
            .any(|(sub_id, _)| contains_shiny(bags, &sub_id)),
    };

    // cache result
    // Have to redo lookup for borrow reasons
    bags.get_mut(id).unwrap().eventually_contains_shiny = Some(success);

    success
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let bag_name_regex = Regex::new(r"(\w+ \w+)").unwrap();
    let get_name = |line: &str| -> String {
        let name_capture = bag_name_regex.captures(line);
        match name_capture {
            Some(caps) => {
                if caps.len() == 2 {
                    String::from(&caps[1])
                } else {
                    panic!("Failed to read bag name for '{}'", &line);
                }
            }
            None => panic!("Failed to match bag name for '{}'", &line),
        }
    };

    let contents_regex = Regex::new(r"(?:(\d+) (\w+ \w+) bag[s]*[,.])+").unwrap();
    let get_contents = |line: &str| -> Vec<(String, usize)> {
        contents_regex
            .captures_iter(line)
            .map(|cap| (String::from(&cap[2]), cap[1].parse::<usize>().unwrap()))
            .collect()
    };

    // Maps string to bag id
    let mut bag_map: HashMap<String, Bag> = HashMap::with_capacity(contents.lines().count());

    // Read all bags into the hash map
    contents.lines().for_each(|line| {
        let id = get_name(&line);
        let contents = get_contents(&line);
        let eventually_contains_shiny = if &id == "shiny gold" {
            Some(true)
        } else {
            None
        };

        bag_map.insert(
            String::from(&id),
            Bag {
                id,
                contents,
                eventually_contains_shiny,
            },
        );
    });

    // Part 1
    // For each bag, determine if it eventually contains the shiny bag
    let keys: Vec<String> = bag_map.keys().map(|id| String::from(id)).collect();
    let count = keys.iter().fold(0, |acc, id| {
        if contains_shiny(&mut bag_map, &id) {
            acc + 1
        } else {
            acc
        }
    });

    println!("Part 1: {}", count - 1); // -1 to skip the shiny bag itself

    // Part 2
    // Iteratively count the bags starting at "shiny gold"
    let mut id_queue: Vec<&str> = vec!["shiny gold"];
    let mut total = 0;
    while let Some(key) = id_queue.pop() {
        let bag = bag_map.get(key).unwrap();

        for (sub_id, count) in bag.contents.iter() {
            for _ in 0..*count {
                id_queue.push(&sub_id);
            }
            total += count;
        }
    }

    println!("Part 2: {}", total);
}
