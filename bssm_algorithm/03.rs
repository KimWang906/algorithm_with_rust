use std::io::stdin;

/*
    Problem : 자연수 A, B가 주어지면 A부터 B까지의 합을 수식과 함께 출력하여라.
    입력 제한 : 1 <= A < B <= 100
    입력 요구사항 : A, B을 한 줄에 공백을 통해 구분하여 입력한다.
    출력 요구사항 : 첫 줄에 더하는 수식과 함께 합을 출력한다.
*/

fn input() -> i32 {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let res = s.trim().parse::<i32>().unwrap();
    res
}

fn main() {
    let value = input();
    let data = (1..value + 1).filter(|x| value != *x && value % x == 0).collect::<Vec<i32>>();
    let data_cp = data.clone();
    let sum: i32 = data_cp.into_iter().sum();
    let res = (0..=(data.len() - 1) as i32).for_each(|x| {
        print!("{} ", data[x as usize]);
        if data[data.len() - 1] == data[x as usize] {
            println!("= {}", sum);
        } else {
            print!("+ ");
        }
    });

    res
}
