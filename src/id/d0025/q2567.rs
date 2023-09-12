// BOJ 2567 [Colored Papers 2]
// Supported by GitHub Copilot

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

    let mut b = [[false; 110]; 110];
    for _ in 0..next(&mut it) {
        let (p, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
        for i in p+1..p+11 {
            for j in q+1..q+11 {
                b[i][j] = true;
            }
        }
    }

    let mut ans = 0;
    for i in 1..102 {
        for j in 1..102 {
            if !b[i][j] {
                if b[i-1][j] { ans += 1; }
                if b[i+1][j] { ans += 1; }
                if b[i][j-1] { ans += 1; }
                if b[i][j+1] { ans += 1; }
            }
        }
    }
    writeln!(so, "{}", ans).unwrap();
}
