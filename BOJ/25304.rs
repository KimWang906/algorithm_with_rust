use std::io::stdin;

fn input_double_val() -> Vec<i32> {
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

fn input_val() -> i32 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let x = input.trim().parse::<i32>().expect("Wrong value");

    x
}

fn main() {
    let x = input_val();
    let n = input_val();
    let mut sum = 0;

    for _ in 0..n {
        let data = input_double_val();
        sum += data[0] * data[1];
    }

    if sum == x {
        println!("Yes");
    } else {
        println!("No");
    }
}