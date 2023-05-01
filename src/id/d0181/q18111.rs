// BOJ 18111 [Minecraft]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
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
    let (n, m, b) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));

    const MAXH: usize = 256;

    let mut hv = vec![0; MAXH+1];
    let mut sum = 0;
    for _ in 0..n*m {
        let h = next::<usize>(&mut it);
        hv[h] += 1;
        sum += h;
    }

    let (mut ans, mut ans_h) = (usize::MAX, 0usize);
    for h in 0..=MAXH {
        if h * n * m > sum + b { break }
        let mut t = 0;
        for (i, &v) in hv.iter().enumerate() {
            t += if i > h { (i - h) * v * 2 } else { (h - i) * v };
        }
        if ans >= t {
            ans = t;
            ans_h = h;
        }
    }

    writeln!(so, "{} {}", ans, ans_h).unwrap();
}
