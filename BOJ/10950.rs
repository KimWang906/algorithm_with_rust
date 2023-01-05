use std::io::stdin;
use std::collections::VecDeque;

fn input_val() -> Vec<i32> {
    let mut data = Vec::new();
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let input_split: Vec<&str> = input.split(' ').collect();
    let val1 = input_split[0].trim().parse::<i32>().expect("Wrong value");
    let val2 = input_split[1].trim().parse::<i32>().expect("Wrong value");
    data.push(val1);
    data.push(val2);

    data
}

fn main() {
    let mut input = String::new();
    let mut result = Vec::new();
    stdin().read_line(&mut input).unwrap();
    let max = input.trim().parse::<i32>().expect("Wrong value");

    for _ in 0..max {
        let data = input_val();
        let sum = data[0] + data[1];
        result.push(sum);
    }

    let mut ord = VecDeque::from(result);

    for _ in 0..max {
        println!("{}", ord.pop_front().unwrap());
    }
}
