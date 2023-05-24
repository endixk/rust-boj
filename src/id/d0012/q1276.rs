// BOJ 1276 [PLATFORME]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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

    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_|
        (next(&mut it), next(&mut it), next(&mut it))).collect::<Vec<(u32, u32, u32)>>();

    let mut ans = 0;
    for (i, &(h1, p1, q1)) in v.iter().enumerate() {
        let (mut hp, mut hq) = (0, 0);
        for (j, &(h2, p2, q2)) in v.iter().enumerate() {
            if i == j { continue; }
            if h1 <= h2 { continue; }
            if p1 >= p2 && p1 < q2 && hp < h2 { hp = h2; }
            if q1 > p2 && q1 <= q2 && hq < h2 { hq = h2; }
        }
        ans += (h1 - hp) + (h1 - hq);
    }
    writeln!(so, "{}", ans).ok();
}
