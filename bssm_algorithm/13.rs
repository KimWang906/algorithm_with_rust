use std::collections::{BTreeMap, HashMap};
use std::io::stdin;

fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    let value = input();
    let value_to_chars = value.chars();
    let mut num_count = BTreeMap::new();
    for i in 0..=9 {
        // init : {0: 0, 1: 0, 2: 0, 3: 0, 4: 0, 5: 0, 6: 0, 7: 0, 8: 0, 9: 0}
        num_count.insert(i, 0);
    }

    for c in value_to_chars {
        if c.is_numeric() {
            let number = c.to_digit(10).unwrap();
            if let Some(value) = num_count.get_mut(&number) {
                *value += 1;
            }
        }
    }

    let binding = num_count.clone();
    let max = binding.values().max().unwrap();

    let binding = num_count.clone();
    let filter: HashMap<&u32, &i32> = binding.iter().filter(|f| *f.1 == *max).collect();

    let res = filter.keys().into_iter().max().unwrap();
    println!("{}", res);
}
