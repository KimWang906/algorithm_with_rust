use std::io::stdin;
use std::collections::HashMap;

/*
    Problem :
        N개의 자연수가 입력되면 각 자연수의 자릿수의 합을 구하고, 그 합이 최대인 자연수를 출력하는 프로그램을 작성하세요.

    ▣ 입력설명 :
        첫 줄에 자연수의 개수 N(3 <= N <= 100)이 주어지고, 그 다음 줄에 N개의 자연수가 주어진다. 각 자연수의 크기는 10,000,000를 넘지 않는다.

    ▣ 출력설명 :
        자릿수의 합이 최대인 자연수를 출력한다. 자리수의 합이 최대인 자연수가 여러개인 경우 그 중 값이 가장 큰 값을 출력합니다.
*/

fn input() -> i32 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let res = input
        .as_mut_str()
        .trim()
        .parse::<i32>()
        .unwrap();

    res
}

fn input_data() -> Vec<String> {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();
    let res: Vec<String> = input
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect();

    res
}

fn main() {
    let input = input();
    let input_data = input_data();
    let binding = input_data.clone();
    let parse: Vec<i32> = binding.iter().map(|s| s.parse::<i32>().unwrap()).collect();
    let mut data = HashMap::new();
    let mut max_value = 0;

    for i in 0..=input - 1 {
        let mut sum = 0;
        for c in input_data[i as usize].chars() {
            if c.is_numeric() {
                sum += c.to_digit(10).unwrap();
            }
        }
        if max_value < sum {
            max_value = sum;
        }
        data.insert(parse[i as usize], sum);
    }
    let filter_value: HashMap<&i32, &u32> = data.iter().filter(|(_x, y)| **y == max_value).collect();
    let binding = filter_value.clone();
    let filter_max_key = binding.iter().max().unwrap();
    
    println!("{}", filter_max_key.0);
}
