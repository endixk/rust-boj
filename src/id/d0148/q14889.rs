// BOJ 14889 [Start and Link]
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

fn calc(s: &Vec<Vec<i32>>, p: &[bool], n: usize) -> i32 {
    let mut ans = 0;
    for i in 0..n-1 {
        for j in i+1..n {
            if p[i] && p[j] { ans += s[i][j]; ans += s[j][i]; }
            if !p[i] && !p[j] { ans -= s[i][j]; ans -= s[j][i]; }
        }
    }
    ans.abs()
}

const INF: i32 = 0x3f3f3f3f;
fn go(s: &Vec<Vec<i32>>, p: &mut [bool], i: usize, c: usize, n: usize) -> i32 {
    if c == n/2 { return calc(s, p, n); }
    if n-i < n/2-c { return INF; }
    let mut ans = INF;
    for j in i..n {
        p[j] = true;
        ans = ans.min(go(s, p, j+1, c+1, n));
        p[j] = false;
    }
    ans
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut s = vec![vec![0; n]; n];
    for i in 0..n { for j in 0..n {
        s[i][j] = next::<i32>(&mut it);
    }}
    let mut p = vec![false; n];
    writeln!(so, "{}", go(&s, &mut p, 0, 0, n)).unwrap();
}
