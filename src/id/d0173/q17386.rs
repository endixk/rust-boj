// BOJ 17386 [Line Segment Intersection]
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

struct Point { x: f64, y: f64 }
struct Line { p1: Point, p2: Point }

fn intersect(l1: &Line, l2: &Line) -> bool {
    let d = (l1.p2.x - l1.p1.x) * (l2.p2.y - l2.p1.y) - (l1.p2.y - l1.p1.y) * (l2.p2.x - l2.p1.x);
    if d == 0.0 { return false; }

    let t = ((l1.p1.y - l2.p1.y) * (l2.p2.x - l2.p1.x) - (l1.p1.x - l2.p1.x) * (l2.p2.y - l2.p1.y)) / d;
    let u = ((l1.p1.y - l2.p1.y) * (l1.p2.x - l1.p1.x) - (l1.p1.x - l2.p1.x) * (l1.p2.y - l1.p1.y)) / d;
    if t < 0.0 || t > 1.0 || u < 0.0 || u > 1.0 { return false; }

    true
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let l1 = Line {
        p1: Point { x: next(&mut it), y: next(&mut it) },
        p2: Point { x: next(&mut it), y: next(&mut it) },
    };
    let l2 = Line {
        p1: Point { x: next(&mut it), y: next(&mut it) },
        p2: Point { x: next(&mut it), y: next(&mut it) },
    };
    writeln!(so, "{}", intersect(&l1, &l2) as u8).unwrap();
}
