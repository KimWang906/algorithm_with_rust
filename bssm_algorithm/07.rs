use std::io::stdin;

fn input() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let input = input();

    for mut c in input.chars() {
        if c.is_ascii() && c != ' ' {
            c.make_ascii_lowercase();
            print!("{}", c);
        }
    }
}
