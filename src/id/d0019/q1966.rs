// BOJ 1966 [Printer Queue]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

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
    for _ in 0..next(&mut it) {
        let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let mut q = VecDeque::new();
        let mut v = Vec::new();
        for i in 0..n {
            let p = next::<usize>(&mut it);
            q.push_back((p, i==m));
            v.push(p);
        }

        v.sort();
        let mut ans = 1;
        while let Some((p, m)) = q.pop_front() {
            if p == *v.last().unwrap() {
                if m {
                    writeln!(so, "{}", ans).unwrap();
                    break;
                }
                v.pop();
                ans += 1;
            } else {
                q.push_back((p, m));
            }
        }
    }
}
