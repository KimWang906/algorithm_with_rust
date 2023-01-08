use std::io::stdin;

fn input() -> i32 {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let res = s.trim().parse::<i32>().unwrap();
    res
}

fn main() {
    let n = input();

    let mut count = 0;
    for i in 1..=n {
        let mut num = i;
        while num > 0 {
            count += 1;
            num /= 10;
        }
    }
    println!("{}", count);
}
