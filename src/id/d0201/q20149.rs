// BOJ 20149 [Line Segment Intersection 3]
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
    fn equal(&self, p: &Point) -> bool {
        (self.x - p.x).abs() < 1e-9 && (self.y - p.y).abs() < 1e-9
    }

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
    fn intersection(&self, l: &Line) -> Option<Point> {
        let d = (self.p2.x - self.p1.x) * (l.p2.y - l.p1.y) - (self.p2.y - self.p1.y) * (l.p2.x - l.p1.x);
        if d == 0.0 { return None; }

        let t = ((self.p1.y - l.p1.y) * (l.p2.x - l.p1.x) - (self.p1.x - l.p1.x) * (l.p2.y - l.p1.y)) / d;
        let u = ((self.p1.y - l.p1.y) * (self.p2.x - self.p1.x) - (self.p1.x - l.p1.x) * (self.p2.y - self.p1.y)) / d;
        if t < 0.0 || t > 1.0 || u < 0.0 || u > 1.0 { return None; }

        Some(Point { x: self.p1.x + t * (self.p2.x - self.p1.x), y: self.p1.y + t * (self.p2.y - self.p1.y) })
    }

    fn parellel(&self, l: &Line) -> bool {
        let d = (self.p2.x - self.p1.x) * (l.p2.y - l.p1.y) - (self.p2.y - self.p1.y) * (l.p2.x - l.p1.x);
        d.abs() < 1e-9
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

    if l1.parellel(&l2) {
        let mut on = Vec::new();
        if l1.p1.on(&l2) { on.push(&l1.p1); }
        if l1.p2.on(&l2) { on.push(&l1.p2); }
        if l2.p1.on(&l1) { on.push(&l2.p1); }
        if l2.p2.on(&l1) { on.push(&l2.p2); }
        if on.len() == 0 {
            writeln!(so, "0").unwrap();
        } else if on.len() == 1 {
            unreachable!();
        } else if on.len() == 2 {
            if on[0].equal(on[1]) {
                writeln!(so, "1").unwrap();
                writeln!(so, "{} {}", on[0].x, on[0].y).unwrap();
            } else {
                writeln!(so, "1").unwrap();
            }
        } else {
            writeln!(so, "1").unwrap();
        }
    } else {
        if l1.p1.on(&l2) {
            writeln!(so, "1").unwrap();
            writeln!(so, "{} {}", l1.p1.x, l1.p1.y).unwrap();
        } else if l1.p2.on(&l2) {
            writeln!(so, "1").unwrap();
            writeln!(so, "{} {}", l1.p2.x, l1.p2.y).unwrap();
        } else if l2.p1.on(&l1) {
            writeln!(so, "1").unwrap();
            writeln!(so, "{} {}", l2.p1.x, l2.p1.y).unwrap();
        } else if l2.p2.on(&l1) {
            writeln!(so, "1").unwrap();
            writeln!(so, "{} {}", l2.p2.x, l2.p2.y).unwrap();
        } else {
            match l1.intersection(&l2) {
                Some(p) => {
                    writeln!(so, "1").unwrap();
                    writeln!(so, "{} {}", p.x, p.y).unwrap();
                },
                None => {
                    writeln!(so, "0").unwrap();
                }
            }
        }
    }
}
