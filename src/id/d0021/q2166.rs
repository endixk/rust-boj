// BOJ 2166 [Area of a Polygon]
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

fn signed_area(x1: i32, y1: i32, x2: i32, y2: i32) -> i64 {
    (x1 as i64) * (y2 as i64) - (x2 as i64) * (y1 as i64)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n: usize = next(&mut it);

    let first = (next::<i32>(&mut it), next::<i32>(&mut it));
    let mut prev = first;
    let mut area: i64 = 0;
    for i in 1..n {
        let curr = (next::<i32>(&mut it), next::<i32>(&mut it));
        area += signed_area(prev.0, prev.1, curr.0, curr.1);
        if i == n - 1 {
            area += signed_area(curr.0, curr.1, first.0, first.1);
        }
        prev = curr;
    }

    writeln!(so, "{:.1}", area.abs() as f64 / 2.0).unwrap();
}
