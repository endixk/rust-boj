// BOJ 9184 [Function Run Fun]
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

fn go(dp: &mut [[[i32; 21]; 21]; 21], a: i32, b: i32, c: i32) -> i32 {
    if a <= 0 || b <= 0 || c <= 0 { return 1; }
    if a > 20 || b > 20 || c > 20 { return go(dp, 20, 20, 20); }
    if dp[a as usize][b as usize][c as usize] != 0 { return dp[a as usize][b as usize][c as usize]; }
    dp[a as usize][b as usize][c as usize] = if a < b && b < c {
        go(dp, a, b, c-1)
            + go(dp, a, b-1, c-1)
            - go(dp, a, b-1, c)
    } else {
        go(dp, a-1, b, c)
            + go(dp, a-1, b-1, c)
            + go(dp, a-1, b, c-1)
            - go(dp, a-1, b-1, c-1)
    };
    dp[a as usize][b as usize][c as usize]
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut dp = [[[0i32; 21]; 21]; 21];
    loop {
        let (a, b, c) = (next::<i32>(&mut it), next::<i32>(&mut it), next::<i32>(&mut it));
        if a == -1 && b == -1 && c == -1 { break; }
        writeln!(so, "w({}, {}, {}) = {}", a, b, c, go(&mut dp, a, b, c)).ok();
    }
}
