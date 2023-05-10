// BOJ 16928 [Snakes and Ladders]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

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

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let lad = (0..n).fold(vec![0; 101], |mut v, _| {
        let (a, b) = (next::<usize>(&mut it), next::<usize>(&mut it));
        v[a] = b; v
    });
    let snk = (0..m).fold(vec![0; 101], |mut v, _| {
        let (a, b) = (next::<usize>(&mut it), next::<usize>(&mut it));
        v[a] = b; v
    });

    let mut d = vec![0; 101];
    let mut q = VecDeque::new();
    q.push_back(1);
    while let Some(x) = q.pop_front() {
        for n in 1..7 {
            if x + n > 100 { continue; }
            let y = if lad[x + n] > 0 { lad[x + n] } else if snk[x + n] > 0 { snk[x + n] } else { x + n };
            if d[y] == 0 { d[y] = d[x] + 1; q.push_back(y); }
        }
        if d[100] > 0 { break; }
    }
    writeln!(so, "{}", d[100]).ok();
}