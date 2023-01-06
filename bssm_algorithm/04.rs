use std::io::stdin;

fn input_val() -> i32 {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let res = s.trim().parse::<i32>().unwrap();
    res
}

fn input() -> Vec<i32> {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    let res: Vec<i32> = s
        .as_mut_str() // 조금 더 빠른 &str로 캐스팅
        .split_whitespace() // ' '로 데이터를 구분해 분류
        .map(|s| s.parse::<i32>().unwrap()) // 반복자를 통해 클로저로 개체 하나마다 타입을 캐스팅한다.
        .collect(); // collection type으로 만들기

    res
}

fn main() {
    let limit = input_val();
    let value = input();
    let binding = value.clone();
    let min_value = binding.iter().min().unwrap();
    let binding = value.clone();
    let max_value = binding.iter().max().unwrap();

    println!("{}", max_value - min_value);
}
