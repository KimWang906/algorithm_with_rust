use std::io::stdin;

fn input() -> Vec<i32> {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    let res = s
        .as_mut_str() // 조금 더 빠른 &str로 캐스팅
        .trim()
        .split("-")
        .map(|s| s.parse::<i32>().unwrap())
        .collect(); // collection type으로 만들기

    res
}

fn main() {
    let data = input();
    let mut year = 0;

    match data[1] / 1000000 {
        1 => {
            year += 1900 + (data[0] / 10000);
            println!("{} M", 2020 - year);
        },
        2 => {
            year += 1900 + (data[0] / 10000);
            println!("{} W", 2020 - year);
        },
        3 => {
            year += 2000 + (data[0] / 10000);
            println!("{} M", 2020 - year);
        },
        4 => {
            year += 1900 + (data[0] / 10000);
            println!("{} W", 2020 - year);
        },
        _ => {}
    }
}
