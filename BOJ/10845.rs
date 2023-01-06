use std::io::stdin;
use std::collections::VecDeque;
fn input_double_val() -> Vec<String> {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let values: Vec<String> = input
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let x = input.trim().parse::<i32>().expect("Wrong value");
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    for _ in 0..x {
        let command = input_double_val();
        match command[0].as_str() {
            "push" => {
                let arg = command[1].trim().parse::<i32>().expect("Wrong Value!");
                queue.push_front(arg);
            }
            "pop" => {
                if queue.is_empty() {
                    result.push(-1);
                } else {
                    result.push(queue.pop_back().unwrap())
                }
            }
            "front" => {
                if queue.is_empty() {
                    result.push(-1);
                } else {
                    result.push(queue[queue.len() - 1])
                }
            }
            "size" => {
                let value = queue.len() as i32;
                result.push(value);
            }
            "empty" => {
                if queue.is_empty() {
                    result.push(1);
                } else {
                    result.push(0);
                }
            },
            "back" => {
                if queue.is_empty() {
                    result.push(-1);
                } else {
                    result.push(queue[0]);
                }
            }
            _ => {}
        }
    }

    result.reverse();

    for _ in 0..result.len() {
        println!("{}", result.pop().unwrap());
    }
}
