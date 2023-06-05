// BOJ 6439 [Intersection]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::cmp::{min, max};

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

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct Point { x: i32, y: i32 }
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if t > 0 { 1 }
    else if t < 0 { -1 }
    else { 0 }
}

struct Line { a: Point, b: Point }
fn intersect(l1: &Line, l2: &Line) -> bool {
    let ab = ccw(&l1.a, &l1.b, &l2.a) * ccw(&l1.a, &l1.b, &l2.b);
    let cd = ccw(&l2.a, &l2.b, &l1.a) * ccw(&l2.a, &l2.b, &l1.b);
    if ab == 0 && cd == 0 {
        let p1 = min(&l1.a, &l1.b);
        let p2 = max(&l1.a, &l1.b);
        let p3 = min(&l2.a, &l2.b);
        let p4 = max(&l2.a, &l2.b);
        return if p1 == p3 || p1 == p4 || p2 == p3 || p2 == p4 { true }
        else { p2 >= p3 && p4 >= p1 }
    }
    ab <= 0 && cd <= 0
}

struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    lines: [Line; 4],
}
impl Rect {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        let (x1, y1, x2, y2) = (min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2));
        let (l1, l2, l3, l4) = (
            Line { a: Point { x: x1, y: y1 }, b: Point { x: x1, y: y2 } },
            Line { a: Point { x: x1, y: y2 }, b: Point { x: x2, y: y2 } },
            Line { a: Point { x: x2, y: y2 }, b: Point { x: x2, y: y1 } },
            Line { a: Point { x: x2, y: y1 }, b: Point { x: x1, y: y1 } },
        );
        Rect { x1, y1, x2, y2, lines: [l1, l2, l3, l4] }
    }
    fn inside(&self, p: &Point) -> bool {
        self.x1 <= p.x && p.x <= self.x2 && self.y1 <= p.y && p.y <= self.y2
    }
    fn intersect(&self, l: &Line) -> bool {
        if self.inside(&l.a) || self.inside(&l.b) { return true }
        for i in 0..4 {
            if intersect(&self.lines[i], l) { return true }
        }
        false
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let (x1, y1, x2, y2) = (next(&mut it), next(&mut it), next(&mut it), next(&mut it));
        let (x3, y3, x4, y4) = (next(&mut it), next(&mut it), next(&mut it), next(&mut it));
        let l = Line { a: Point { x: x1, y: y1 }, b: Point { x: x2, y: y2 } };
        let r = Rect::new(x3, y3, x4, y4);
        writeln!(so, "{}", if r.intersect(&l) { "T" } else { "F" }).ok();
    }
}
