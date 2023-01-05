use std::io::stdin;

fn input_double_val() -> Vec<i32> {
    let mut data = Vec::new();
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let input_split: Vec<&str> = input.split(' ').collect();

    let rev_str1: String = input_split[0].chars().rev().collect();
    let rev_str2: String = input_split[1].chars().rev().collect();

    let val1 = rev_str1.trim().parse::<i32>().expect("Wrong value");
    let val2 = rev_str2.trim().parse::<i32>().expect("Wrong value");
    data.push(val1);
    data.push(val2);

    data
}

fn main() {
    let input = input_double_val();
    let max = input.into_iter().max().unwrap();
    println!("{max}");
}