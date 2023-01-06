use std::io::stdin;

/*
    Problem : 자연수 n이 입력되면 1부터 n까지의 수 중 M의 배수합을 출력하는 프로그램을 작성하시오.
    입력 제한 : 3 <= M < N <= 1000
    입력 요구사항 : N, M을 한 줄에 입력한다.
    출력 요구사항 : 첫 줄에 M의 배수합을 출력한다.
*/

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
    let value = input();
    let mut res = 0;
    let _sum = (0..=value[0]).for_each(|x| {
        if x % value[1] == 0 {
            res += x;
        }
    });

    println!("{}", res);
}
