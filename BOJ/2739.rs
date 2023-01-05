use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let num = input.trim().parse::<u8>().expect("Wrong Value --> input type i32");

    for i in 1..10 {
        println!("{num} * {i} = {}", num * i);
    }
}
