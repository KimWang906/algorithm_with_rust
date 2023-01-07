use std::io::{stdin, stdout, BufWriter, Write};

fn input() -> i32 {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();

    n
}

fn main() {
    let n = input();
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut measurements = vec![0; n as usize + 1];

    for i in 1..=n {
        let mut count = 0;
        for j in 1..=(i as f32).sqrt() as i32 {
            if i % j == 0 {
                count += 1;

                if j * j != i {
                    count += 1;
                }
            }
        }
        measurements[i as usize] = count;
    }

    for i in 1..=n {
        write!(out, "{} ", measurements[i as usize]).unwrap();
    }

    writeln!(out).unwrap();
}
