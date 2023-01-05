use std::io::{stdin, stdout, Write, BufWriter};
use std::collections::VecDeque;

fn input_double_val() -> Vec<u16> {
    let mut data = Vec::new();
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let input_split: Vec<&str> = input.split(' ').collect();
    let val1 = input_split[0].trim().parse::<u16>().expect("Wrong value");
    let val2 = input_split[1].trim().parse::<u16>().expect("Wrong value");
    data.push(val1);
    data.push(val2);

    data
}

fn input_val() -> i32 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let x = input.trim().parse::<i32>().expect("Wrong value");

    x
}

fn main() {
    let stdout = stdout(); // 2
    let mut out = BufWriter::new(stdout.lock()); // 3
    let mut result = Vec::new();

    let t = input_val();

    for _ in 0..t {
        let data = input_double_val();
        let sum = data[0] + data[1];
        result.push(sum);
    }

    let mut ord = VecDeque::from(result);

    for _ in 0..t {
        writeln!(out, "{}", ord.pop_front().unwrap());
    }
}
