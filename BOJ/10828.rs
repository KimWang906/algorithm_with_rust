use std::io::stdin;

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
    let mut stack = Vec::new();
    let mut result = Vec::new();

    for _ in 0..x {
        let command = input_double_val();
        match command[0].as_str() {
            "push" => {
                let arg = command[1].trim().parse::<i32>().expect("Wrong Value!");
                stack.push(arg);
            }
            "pop" => {
                if stack.is_empty() {
                    result.push(-1);
                } else {
                    result.push(stack.pop().unwrap())
                }
            }
            "top" => {
                if stack.is_empty() {
                    result.push(-1);
                } else {
                    result.push(stack[stack.len() - 1])
                }
            }
            "size" => {
                let value = stack.len() as i32;
                result.push(value);
            }
            "empty" => {
                if stack.is_empty() {
                    result.push(1);
                } else {
                    result.push(0);
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
