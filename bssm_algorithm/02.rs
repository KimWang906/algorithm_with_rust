use std::io::stdin;

/*
    Problem : 자연수 A, B가 주어지면 A부터 B까지의 합을 수식과 함께 출력하여라.
    입력 제한 : 1 <= A < B <= 100
    입력 요구사항 : A, B을 한 줄에 공백을 통해 구분하여 입력한다.
    출력 요구사항 : 첫 줄에 더하는 수식과 함께 합을 출력한다.
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
    let sum: i32 = (value[0]..=value[1]).sum();
    let res = (value[0]..=value[1]).for_each(|x| {
        print!("{} ", x);
        if x != value[1] {
            print!("+ ");
        } else {
            println!("= {sum}");
        }
    });
    res
}
