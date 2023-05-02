// BOJ 9244 [Pinball]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::collections::BTreeSet;

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

struct Point { x: i64, y: i64, e: bool, f: bool, i: usize }
impl Eq for Point {}
impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.e == other.e && self.f == other.f
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x == other.x {
            if self.e == other.e {
                if self.e {
                    other.y.cmp(&self.y)
                } else {
                    self.y.cmp(&other.y)
                }
            } else {
                self.e.cmp(&other.e)
            }
        } else {
            self.x.cmp(&other.x)
        }
    }
}
impl PartialOrd<Self> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Clone for Point {
    fn clone(&self) -> Self {
        Point { x: self.x, y: self.y, e: self.e, f: self.f, i: self.i }
    }
}

struct Line { p1: Point, p2: Point }
impl Line {
    fn y_at_x(&self, x: i64) -> f64 {
        (self.p2.y - self.p1.y) as f64 / (self.p2.x - self.p1.x) as f64 * (x - self.p1.x) as f64 + self.p1.y as f64
    }
}
impl Eq for Line {}
impl PartialEq<Self> for Line {
    fn eq(&self, other: &Self) -> bool {
        self.y_at_x(unsafe{T} ) == other.y_at_x(unsafe{T} )
    }
}
impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        let s = self.y_at_x(unsafe{T} );
        let o = other.y_at_x(unsafe{T} );
        if s < o { Ordering::Less }
        else if s > o { Ordering::Greater }
        else { Ordering::Equal }
    }
}
impl PartialOrd<Self> for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Clone for Line {
    fn clone(&self) -> Self {
        Line { p1: self.p1.clone(), p2: self.p2.clone() }
    }
}

static mut T: i64 = 0;

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n: usize = next(&mut it);

    let mut points = Vec::new();
    let mut lines = Vec::new();
    (0..n).for_each(|i| {
        let (x1, y1, x2, y2) = (
            next::<i64>(&mut it),
            next::<i64>(&mut it),
            next::<i64>(&mut it),
            next::<i64>(&mut it)
        );
        let e1 = x1 > x2;
        let e2 = !e1;
        let f1 = y1 < y2;
        let f2 = !f1;
        let p1 = Point { x: x1, y: y1, e: e1, f: f1, i };
        let p2 = Point { x: x2, y: y2, e: e2, f: f2, i };
        points.push(p1.clone());
        points.push(p2.clone());
        if e1 { lines.push(Line { p2, p1 }); }
        else { lines.push(Line { p1, p2 }); }
    });

    const SINK: usize = 111111;
    let mut nxt = vec![SINK; n];
    let mut ins = vec![false; n];

    // sweeping by x
    points.sort();
    let mut set = BTreeSet::<Line>::new();
    for point in points {
        unsafe { T = point.x; }
        if point.f {
            if let Some(p) = set.range(..lines[point.i].clone()).last() {
                nxt[point.i] = p.p1.i;
            }
        }
        if ins[point.i] {
            set.remove(&lines[point.i]);
        } else {
            set.insert(lines[point.i].clone());
            ins[point.i] = true;
        }
    }

    // find line with max y
    let x = next(&mut it);
    let (mut my, mut mi) = (f64::MIN, SINK);
    for (i, line) in lines.iter().enumerate() {
        if line.p1.x.min(line.p2.x) > x || line.p1.x.max(line.p2.x) < x { continue; }
        let y = line.y_at_x(x);
        if y > my {
            my = y;
            mi = i;
        }
    }
    if mi == SINK { writeln!(so, "{}", x).ok(); return; }

    // follow the path
    while nxt[mi] != SINK { mi = nxt[mi] }
    writeln!(so, "{}", if lines[mi].p1.f { lines[mi].p1.x } else { lines[mi].p2.x }).ok();
}
