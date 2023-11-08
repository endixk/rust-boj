// BOJ 7898 [Euro Efficiency]
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

fn get(dp: &mut [i32], a: &[usize], x: usize) -> i32 {
    if dp[10000 + x] != -1 { return dp[10000 + x]; }
    let mut v = vec![false; 20000];
    let mut q = std::collections::VecDeque::new();
    v[10000] = true; q.push_back(10000);
    let mut d = 0;
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let c = q.pop_front().unwrap();
            dp[c] = d;
            if c == 10000 + x { return d; }
            for &k in a {
                if !v[c + k] {
                    v[c + k] = true;
                    q.push_back(c + k);
                }
                if !v[c - k] {
                    v[c - k] = true;
                    q.push_back(c - k);
                }
            }
        }
        d += 1;
    }
    0
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let a = (0..6).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
        let (mut c, mut m) = (0, 0);
        let mut dp = vec![-1; 20000];
        for i in 1..101 {
            let x = get(&mut dp, &a, i);
            c += x;
            if x > m { m = x; }
        }
        writeln!(so, "{:.2} {}", c as f64 / 100.0, m)?;
    }

    Ok(())
}
