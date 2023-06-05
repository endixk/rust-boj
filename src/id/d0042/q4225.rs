// BOJ 4225 [Trash Removal]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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
}
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64;
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}

fn graham(mut points: Vec<Point>) -> Vec<Point> {
    // find the lowest point
    let mut loc = 0;
    for i in 1..points.len() {
        if points[i].y < points[loc].y || (points[i].y == points[loc].y && points[i].x < points[loc].x) {
            loc = i;
        }
    }
    let ori = points[loc].clone();

    // sort the points by polar angle
    points.sort_by(|a, b| {
        ccw(&ori, b, a).cmp(&0)
            .then(a.y.cmp(&b.y))
            .then(a.x.cmp(&b.x))
    });

    // find ori in the sorted points and swap it to the first
    let mut loc = 0;
    for i in 0..points.len() {
        if points[i] == ori {
            loc = i;
            break;
        }
    }
    points.swap(0, loc);

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

fn dist(line: (&Point, &Point), point: &Point) -> f64 {
    if line.0.x == line.1.x {
        return (line.0.x - point.x).pow(2) as f64;
    }
    let (x1, y1) = (line.0.x as f64, line.0.y as f64);
    let (x2, y2) = (line.1.x as f64, line.1.y as f64);
    let (p, q) = (point.x as f64, point.y as f64);
    let (a, b, c) = (y2 - y1, x1 - x2, x2 * y1 - x1 * y2);
    (a * p + b * q + c).powi(2) / (a * a + b * b)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut case = 1;
    loop {
        let n = next::<usize>(&mut it);
        if n == 0 { break; }

        let points = (0..n).map(|_| {
            Point {
                x: next::<i32>(&mut it),
                y: next::<i32>(&mut it),
            }
        }).collect::<Vec<_>>();
        let hull = graham(points);

        let mut ans = 1e12;
        for i in 0..hull.len() {
            let mut d = 0.0;
            for j in 0..hull.len() {
                let x = dist((&hull[i], &hull[(i + 1) % hull.len()]), &hull[j]);
                if x > d { d = x; }
            }
            if d < ans { ans = d; }
        }

        writeln!(so, "Case {}: {:.2}", case, (ans.sqrt() * 100.0).ceil() / 100.0).unwrap();
        case += 1;
    }
}
