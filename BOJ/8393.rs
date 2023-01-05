use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let num = input.trim().parse::<i32>().expect("Wrong value");
    let sum: i32 = (0..=num).into_iter().sum(); // Chaning methods and functional style
    println!("{}", sum);
}