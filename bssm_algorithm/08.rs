use std::io::stdin;

fn input() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let res = s.trim().to_owned();

    res
}

fn main() {
    let input = input();

    let mut stack = Vec::new();

    for c in input.chars() {
        if c == '(' {
            stack.push(c);
        } else if c == ')' {
            if stack.is_empty() || stack.pop().unwrap() != '(' {
                println!("NO");
                return;
            }
        }
    }

    if stack.is_empty() {
        println!("YES");
    } else {
        println!("NO");
    }
}
