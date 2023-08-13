use std::io::{self, Read, Write};
fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}
fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let mut dp = [false, true, false, false, false, false, false];
        for _ in 0..next(&mut it) {
            let (op1, v1, op2, v2) = (
                next::<char>(&mut it), next::<usize>(&mut it),
                next::<char>(&mut it), next::<usize>(&mut it)
            );

            let mut tp = [false; 7];
            for i in 0..7 {
                if dp[i] {
                    match op1 {
                        '+' => tp[(i + v1) % 7] = true,
                        _ => tp[(i * v1) % 7] = true,
                    }
                    match op2 {
                        '+' => tp[(i + v2) % 7] = true,
                        _ => tp[(i * v2) % 7] = true,
                    }
                }
            }
            dp = tp;
        }
        writeln!(so, "{}", if dp[0] { "LUCKY" } else { "UNLUCKY" }).ok();
    }
}