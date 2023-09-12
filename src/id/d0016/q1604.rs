// BOJ 1604 [Square Cutting]
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

struct Point { x: f64, y: f64 }
struct Line { p1: Point, p2: Point }
impl Point {
    fn equal(&self, p: &Point) -> bool {
        (self.x - p.x).abs() < 1e-9 && (self.y - p.y).abs() < 1e-9
    }
}
impl Line {
    fn sect(&self, l: &Line) -> Option<Point> {
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

fn ccw(x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64) -> i8 {
    let t = (x1 * y2 + x2 * y3 + x3 * y1) - (x2 * y1 + x3 * y2 + x1 * y3);
    return if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}
fn splits(x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
    let line = Line { p1: Point { x: x1 as f64, y: y1 as f64 }, p2: Point { x: x2 as f64, y: y2 as f64 } };
    if line.sect(&Line { p1: Point { x: -10.0, y: -10.0 }, p2: Point { x: 10.0, y:-10.0 }}).is_none() &&
        line.sect(&Line { p1: Point { x: -10.0, y: 10.0 }, p2: Point { x: 10.0, y: 10.0 }}).is_none() &&
        line.sect(&Line { p1: Point { x: -10.0, y: -10.0 }, p2: Point { x: -10.0, y: 10.0 }}).is_none() &&
        line.sect(&Line { p1: Point { x: 10.0, y: -10.0 }, p2: Point { x: 10.0, y: 10.0 }}).is_none() {
        return false;
    }
    let c1 = ccw(x1, y1, x2, y2, -10, -10);
    let c2 = ccw(x1, y1, x2, y2, -10, 10);
    let c3 = ccw(x1, y1, x2, y2, 10, -10);
    let c4 = ccw(x1, y1, x2, y2, 10, 10);
    return (c1 * c2 < 0) || (c1 * c3 < 0) || (c1 * c4 < 0) ||
        (c2 * c3 < 0) || (c2 * c4 < 0) || (c3 * c4 < 0);
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut ans = 1;
    let mut lines = Vec::new();
    for _ in 0..n {
        let (x1, y1, x2, y2) = (
            next::<i64>(&mut it), next::<i64>(&mut it),
            next::<i64>(&mut it), next::<i64>(&mut it));
        if splits(x1, y1, x2, y2) {
            ans += 1;
        }
        lines.push(Line { p1: Point { x: x1 as f64, y: y1 as f64 }, p2: Point { x: x2 as f64, y: y2 as f64 } });
    }
    for i in 0..n-1 {
        for j in i+1..n {
            if lines[i].parellel(&lines[j]) { continue; }
            if let Some(p) = lines[i].sect(&lines[j]) {
                if p.x.abs() < 10.0 - 1e-9 && p.y.abs() < 10.0 - 1e-9 {
                    ans += 1;
                }
            }
        }
    }
    writeln!(so, "{}", ans).unwrap();
}
