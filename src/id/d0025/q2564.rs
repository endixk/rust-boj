// BOJ 2564 [Security Guard]
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

// dir) 1: N, 2: S, 3: W, 4: E
// off) N/S: from W, E/W: from N
fn dist(h: i32, v: i32, dir1: i32, off1: i32, dir2: i32, off2: i32) -> i32 {
    let (dir1, dir2, off1, off2) = if dir1 < dir2 {
        (dir1, dir2, off1, off2)
    } else {
        (dir2, dir1, off2, off1)
    };
    return match (dir1, dir2) {
        (1, 1) => { (off1 - off2).abs() },
        (1, 2) => { (off1 + off2 + v).min(h - off1 + h - off2 + v) }
        (1, 3) => { off1 + off2 }
        (1, 4) => { h - off1 + off2 }
        (2, 2) => { (off1 - off2).abs() },
        (2, 3) => { off1 + v - off2 },
        (2, 4) => { h - off1 + v - off2 },
        (3, 3) => { (off1 - off2).abs() },
        (3, 4) => { (off1 + off2 + h).min(v - off1 + v - off2 + h) },
        (4, 4) => { (off1 - off2).abs() },
        _ => unreachable!()
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (h, v) = (next::<i32>(&mut it), next::<i32>(&mut it));
    let n = next::<usize>(&mut it);
    let a = (0..n).map(|_| (next::<i32>(&mut it), next::<i32>(&mut it))).collect::<Vec<_>>();
    let b = (next::<i32>(&mut it), next::<i32>(&mut it));

    let mut ans = 0;
    for (dir, off) in a {
        ans += dist(h, v, dir, off, b.0, b.1);
    }
    writeln!(so, "{}", ans).unwrap();
}
