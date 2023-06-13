// BOJ 20437 [String Game 2]
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

    for _ in 0..next(&mut it) {
        let (s, k) = (next::<String>(&mut it), next::<usize>(&mut it));
        let mut v = vec![vec![]; 26];

        let mut flag = false;
        for (i, &c) in s.as_bytes().into_iter().enumerate() {
            let c = (c - b'a') as usize;
            v[c].push(i);
            if !flag && v[c].len() == k { flag = true; }
        }

        if !flag { writeln!(so, "-1").ok(); continue; }
        let (mut mn, mut mx) = (s.len(), 0);
        for locs in v {
            if locs.len() < k { continue; }
            for i in 0..locs.len() - k + 1 {
                mn = mn.min(locs[i + k - 1] - locs[i] + 1);
                mx = mx.max(locs[i + k - 1] - locs[i] + 1);
            }
        }
        writeln!(so, "{} {}", mn, mx).ok();
    }
}
