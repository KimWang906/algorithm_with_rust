use std::io::stdin;
/*
    136

    1: 9 * 1 = 9
    2: 90 * 2 = 180
    - 99 - 10 = 89 + 1 = 90

    1: 9 * 10^0 = 9;
    2: (9 * 10^1) * 2 = 180
    3: 100 ~ 999
    - 999 - 100 = 899 + 1 = 900 * 3 = 2700
    3: 9 * 10^2 * 3

    N: 9 * 10^(n-1) * n

    isPass : 다음 자릿수로 넘어갈 수 있는가?
    
    i = 1 2 3 …

    ex 1) 27

    res = 0;
    status = 0;

    자릿수 올리는 반복문(1 ~ ?)

    isPass = pow, 러스트에서는 문자열로 캐스팅 한 후 해당 길이를 재서 확인

    만약 27 >= isPass <— 다음 자릿수로 넘어갈 수 있을 경우
    ch = 9 * 10^(n-1); <— 해당 자릿수의 개수
    res += ch * i <— 앞에서 구한 값에 자릿수를 곱한다.
    status += ch <— 이미 구한 자릿수의 개수가 중복 합산이 되지 않도록 27에서 9를 뺄 때 사용

    res = (27 - 9) * 2 + res <— 결과를 합산
*/

fn input() -> i32 {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let res = s.trim().parse::<i32>().unwrap();
    res
}

fn main() {
    let value = input();
    let binding = value.to_string();
    let str = binding.as_str();
    let mut middle = 0;
    let mut res = 0; // result
    for i in 1..str.len() { // 자릿수 반복 변수 i
        let mut count = 0;
        count = 9 * (10 as i32).pow((i - 1) as u32);
        middle += count;
        res += count * (i as i32);
    }

    res = (value - middle) * (str.len() as i32) + res;

    println!("{res}");
}
