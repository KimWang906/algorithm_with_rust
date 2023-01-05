use std::io::stdin;

fn input_double_val() -> i32 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let data = input.trim().parse::<i32>().expect("Wrong value");

    data
}

fn main() {
    let mut data = Vec::new();

    for _ in 0..9 {
        let input = input_double_val();
        data.push(input);
    }

    let mut max = data[0];
    let mut max_index = 0;

    for (index, &x) in data.iter().enumerate() {
        if x > max {
            max = x;
            max_index = index;
        }
    }

    println!("{max}\n{}", max_index + 1);
}