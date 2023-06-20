// BOJ 3679 [Simple Polygon]
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

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
    i: usize,
}
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64;
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}
fn dist(a: &Point, b: &Point) -> i64 {
    ((a.x - b.x) as i64).pow(2) + ((a.y - b.y) as i64).pow(2)
}
fn poly(mut points: Vec<Point>) -> Vec<Point> {
    // find the lowest point
    let mut loc = 0;
    for i in 1..points.len() {
        if points[i].y < points[loc].y || (points[i].y == points[loc].y && points[i].x < points[loc].x) {
            loc = i;
        }
    }
    points.swap(0, loc);
    let ori = points[0].clone();

    // sort the points by polar angle
    points.sort_by(|a, b| {
        ccw(&ori, b, a).cmp(&0)
            .then(dist(&ori, a).cmp(&dist(&ori, b)))
    });

    points
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let mut v = (0..next(&mut it)).map(|i|
            Point { x: next(&mut it), y: next(&mut it), i }
        ).collect::<Vec<_>>();
        v = poly(v);

        let mut n = v.len();
        while ccw(&v[0], &v[n-1], &v[n-2]) == 0 {
            n -= 1;
        }
        (n-1..v.len()).rev().for_each(|i| { write!(so, "{} ", v[i].i).unwrap(); });
        (0..n-1).for_each(|i| { write!(so, "{} ", v[i].i).unwrap(); });

        writeln!(so).unwrap();
    }
}
