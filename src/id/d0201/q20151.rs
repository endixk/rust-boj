// BOJ 20151 [Line Segment Intersection 5]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::collections::BTreeSet;

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

fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i32 {
    let ret = (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x);
    if ret > 0 { 1 } else if ret < 0 { -1 } else { 0 }
}

struct Point { x: i128, y: i128, e: bool, i: usize }
impl Point {
    fn eq(&self, p: &Point) -> bool {
        self.x == p.x && self.y == p.y
    }
}
struct Line { p1: Point, p2: Point }
impl Line {
    fn intersect(&self, l: &Line) -> bool {
        let ccw1 = ccw(&self.p1, &self.p2, &l.p1);
        let ccw2 = ccw(&self.p1, &self.p2, &l.p2);
        let ccw3 = ccw(&l.p1, &l.p2, &self.p1);
        let ccw4 = ccw(&l.p1, &l.p2, &self.p2);
        if ccw1 * ccw2 == 0 && ccw3 * ccw4 == 0 {
            return if ccw1 == 0 && ccw2 == 0 && ccw3 == 0 && ccw4 == 0 {
                (self.p1.x.max(self.p2.x)).min(l.p1.x.max(l.p2.x)) > (self.p1.x.min(self.p2.x)).max(l.p1.x.min(l.p2.x))
            } else { false }
        }
        ccw1 * ccw2 <= 0 && ccw3 * ccw4 <= 0
    }
    fn y_at_x(&self, x: i128) -> i128 {
        let dx = self.p2.x - self.p1.x;
        let dy = self.p2.y - self.p1.y;
        let t = x - self.p1.x;
        self.p1.y * dx + dy * t
    }
}

impl Eq for Point {}
impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other) && self.e == other.e
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then(other.e.cmp(&self.e)).then(self.y.cmp(&other.y))
    }
}
impl PartialOrd<Self> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Clone for Point {
    fn clone(&self) -> Self {
        Point { x: self.x, y: self.y, e: self.e, i: self.i }
    }
}

static mut T: i128 = 0;
static mut OFFSET: i128 = 1;

impl Eq for Line {}
impl PartialEq<Self> for Line {
    fn eq(&self, other: &Self) -> bool {
        self.y_at_x(unsafe{T} ) == other.y_at_x(unsafe{T} ) &&
        self.y_at_x(unsafe{T} + unsafe{OFFSET} ) == other.y_at_x(unsafe{T} + unsafe{OFFSET} )
    }
}
impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        return if self.y_at_x(unsafe{T} ) == other.y_at_x(unsafe{T} ) {
            self.y_at_x(unsafe{T} + unsafe{OFFSET} ) .cmp(&other.y_at_x(unsafe{T} + unsafe{OFFSET} ) )
        } else {
            self.y_at_x(unsafe{T} ) .cmp(&other.y_at_x(unsafe{T} ) )
        }
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let mut points = Vec::new();
    for i in 0..next(&mut it) {
        let x1 = next(&mut it);
        let y1 = next(&mut it);
        let x2 = next(&mut it);
        let y2 = next(&mut it);
        if x1 < x2 {
            points.push(Point { x: x1, y: y1, e: false, i });
            points.push(Point { x: x2, y: y2, e: true, i });
        } else {
            points.push(Point { x: x2, y: y2, e: false, i });
            points.push(Point { x: x1, y: y1, e: true, i });
        }

    }

    let mut yp = false;
    for pair in points.chunks(2) {
        if pair[0].x == pair[1].x {
            yp = true;
        }
    }
    if yp {
        let mut kv = [false; 222222];
        for pair in points.chunks(2) {
            if pair[0].y != pair[1].y {
                let k = ((pair[0].x - pair[1].x) / (pair[1].y - pair[0].y)).abs();
                kv[k as usize] = true;
            }
        }
        let mut k: i128 = 222221;
        for i in (1..222222).rev() {
            if !kv[i] {
                k = i as i128;
                break;
            }
        }

        let mut new_points = Vec::new();
        for pair in points.chunks(2) {
            let x1 = pair[0].x + k * pair[0].y;
            let y1 = pair[0].y - k * pair[0].x;
            let x2 = pair[1].x + k * pair[1].y;
            let y2 = pair[1].y - k * pair[1].x;
            if x1 < x2 {
                new_points.push(Point { x: x1, y: y1, e: false, i: pair[0].i });
                new_points.push(Point { x: x2, y: y2, e: true, i: pair[1].i });
            } else {
                new_points.push(Point { x: x2, y: y2, e: false, i: pair[1].i });
                new_points.push(Point { x: x1, y: y1, e: true, i: pair[0].i });
            }
        }
        points = new_points;
    }

    let mut lines = Vec::new();
    for pair in points.chunks(2) {
        lines.push(Line { p1: pair[0].clone(), p2: pair[1].clone() });
    }

    // Shamos-Hoey algorithm
    points.sort();
    let mut set = BTreeSet::<Line>::new();
    for point in points {
        if point.e {
            unsafe { T = point.x; OFFSET = -1 }
            if let Some(p) = set.range(..lines[point.i].clone()).last() {
                if let Some(q) = set.range(lines[point.i].clone()..).skip(1).next() {
                    if p.intersect(q) {
                        writeln!(so, "1").unwrap();
                        return;
                    }
                }
            }
            set.remove(&lines[point.i]);
        } else {
            unsafe { T = point.x; OFFSET = 1 }
            if let Some(p) = set.range(..lines[point.i].clone()).last() {
                if p.intersect(&lines[point.i]) {
                    writeln!(so, "1").unwrap();
                    return;
                }
            }
            if let Some(p) = set.range(lines[point.i].clone()..).next() {
                if p.intersect(&lines[point.i]) {
                    writeln!(so, "1").unwrap();
                    return;
                }
            }
            if !set.insert(lines[point.i].clone()) {
                writeln!(so, "0").unwrap();
                return;
            }
        }
    }

    writeln!(so, "0").unwrap();
}
