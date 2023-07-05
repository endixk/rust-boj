// BOJ 3878 [Separate Points]
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

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            ccw(&ORI, other, self).cmp(&0)
                .then(dist(&ORI, self).cmp(&dist(&ORI, other)))
        }
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64;
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}
fn dist(a: &Point, b: &Point) -> i64 {
    ((a.x - b.x) as i64).pow(2) + ((a.y - b.y) as i64).pow(2)
}
fn within(a: i32, b: i32, x: i32) -> bool {
    return if a < b { a <= x && x <= b } else { b <= x && x <= a }
}
fn inside(poly: &Vec<Point>, p: &Point) -> bool {
    let mut cnt = 0;
    for i in 0..poly.len() {
        let j = (i+1) % poly.len();
        let (x, y) = if poly[i].y < poly[j].y { (&poly[i], &poly[j]) } else { (&poly[j], &poly[i]) };

        if ccw(x, y, p) == 0 && within(x.x, y.x, p.x) && within(x.y, y.y, p.y) {
            return true; // p is on the line
        }
        if x.y <= p.y && p.y < y.y && ccw(x, y, p) > 0 {
            cnt += 1;
        }
    }
    cnt & 1 == 1
}

static mut ORI: Point = Point { x: 0, y: 0 };
fn graham(mut points: Vec<Point>) -> Vec<Point> {
    // find the lowest point
    let mut loc = 0;
    for i in 1..points.len() {
        if points[i].y < points[loc].y || (points[i].y == points[loc].y && points[i].x < points[loc].x) {
            loc = i;
        }
    }
    points.swap(0, loc);
    unsafe { ORI = points[0].clone(); }

    // sort the points by polar angle
    points.sort_unstable();

    // find the convex hull
    let mut hull = Vec::new();
    hull.push(points[0]);
    hull.push(points[1]);
    for i in 2..points.len() {
        while hull.len() >= 2 {
            let a = hull.pop().unwrap();
            let b = hull.last().unwrap();
            if ccw(b, &a, &points[i]) > 0 {
                hull.push(a);
                break;
            }
        }
        hull.push(points[i]);
    }

    hull
}

use std::cmp::{min, max};
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    'T: for _ in 0..next(&mut it) {
        let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let w = (0..n).map(|_| Point { x: next(&mut it), y: next(&mut it) }).collect::<Vec<_>>();
        let w = if w.len() > 2 { graham(w) } else { w };
        let b = (0..m).map(|_| Point { x: next(&mut it), y: next(&mut it) }).collect::<Vec<_>>();
        let b = if b.len() > 2 { graham(b) } else { b };

        for &p in &w {
            if inside(&b, &p) {
                writeln!(so, "NO").unwrap();
                continue 'T;
            }
        }
        for &p in &b {
            if inside(&w, &p) {
                writeln!(so, "NO").unwrap();
                continue 'T;
            }
        }
        if w.len() == 1 || b.len() == 1 {
            writeln!(so, "YES").unwrap();
            continue 'T;
        }
        for i in 0..w.len() {
            let lw = Line { a: w[i].clone(), b: w[(i+1)%w.len()].clone() };
            for j in 0..b.len() {
                let lb = Line { a: b[j].clone(), b: b[(j+1)%b.len()].clone() };
                if intersect(&lw, &lb) {
                    writeln!(so, "NO").unwrap();
                    continue 'T;
                }
            }
        }
        writeln!(so, "YES").unwrap();
    }
}
