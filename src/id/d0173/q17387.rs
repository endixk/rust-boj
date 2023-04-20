// BOJ 17387 [Line Segment Intersection 2]
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
impl Point {
    fn dist(&self, p: &Point) -> f64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn on(&self, l: &Line) -> bool {
        let d1 = self.dist(&l.p1);
        let d2 = self.dist(&l.p2);
        let d3 = l.p1.dist(&l.p2);
        (d1 + d2 - d3).abs() < 1e-9
    }
}
impl Line {
    fn intersect(&self, l: &Line) -> bool {
        let d = (self.p2.x - self.p1.x) * (l.p2.y - l.p1.y) - (self.p2.y - self.p1.y) * (l.p2.x - l.p1.x);
        if d == 0.0 { return false; }

        let t = ((self.p1.y - l.p1.y) * (l.p2.x - l.p1.x) - (self.p1.x - l.p1.x) * (l.p2.y - l.p1.y)) / d;
        let u = ((self.p1.y - l.p1.y) * (self.p2.x - self.p1.x) - (self.p1.x - l.p1.x) * (self.p2.y - self.p1.y)) / d;
        if t < 0.0 || t > 1.0 || u < 0.0 || u > 1.0 { return false; }

        true
    }
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

    if l1.p1.on(&l2) || l1.p2.on(&l2) || l2.p1.on(&l1) || l2.p2.on(&l1) {
        writeln!(so, "1").unwrap();
    } else {
        writeln!(so, "{}", l1.intersect(&l2) as u8).unwrap();
    }
}
