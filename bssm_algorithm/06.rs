use std::io::stdin;

fn input() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let s = input();
    let str = s.as_str();
    let mut count = 0;
    let mut number = 0;
    for c in str.chars() {
        if c.is_numeric() {
            number = number * 10 + c.to_digit(10).unwrap();
        }
    }

    (1..=number + 1).into_iter().for_each(|x| {
        if number % x == 0 {
            count += 1;
        }
    });

    println!("{}", number);
    println!("{}", count);
}
