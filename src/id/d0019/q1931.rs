// BOJ 1931 [Conference Room Assignment]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n = it.by_ref().next().unwrap().parse().unwrap();

    let mut v = Vec::new();
    for _ in 0..n {
        let st: i32 = it.by_ref().next().unwrap().parse().unwrap();
        let ed: i32 = it.by_ref().next().unwrap().parse().unwrap();
        v.push((ed, st));
    }

    v.sort();
    let mut ans = 0;
    let mut last = 0;
    for (ed, st) in v {
        if st >= last {
            ans += 1;
            last = ed;
        }
    }

    writeln!(so, "{}", ans).unwrap();
}
