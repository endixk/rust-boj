// BOJ 2626 [Helipad]
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


#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}
fn dist(a: &Point, b: &Point) -> f64 {
    (a.x - b.x).hypot(a.y - b.y)
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
struct IPoint {
    x: i32,
    y: i32,
}
fn ccw(a: &IPoint, b: &IPoint, c: &IPoint) -> i8 {
    let t = (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64;
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}
fn graham(mut points: Vec<IPoint>) -> Vec<Point> {
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

    hull.into_iter().map(|p| Point { x: p.x as f64, y: p.y as f64 }).collect()
}

const ITER: usize = 1000;
const RTO: f64 = 0.1; // initial ratio
const FAC: f64 = 0.995; // ratio factor
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_| {
        let x = next::<i32>(&mut it);
        let y = next::<i32>(&mut it);
        IPoint { x, y }
    }).collect::<Vec<_>>();

    let v = graham(v);
    let n = v.len();

    let mut c = Point {
        x: v.iter().map(|p| p.x).sum::<f64>() / n as f64,
        y: v.iter().map(|p| p.y).sum::<f64>() / n as f64,
    };
    let mut r = RTO;
    for i in 0..=ITER {
        // find the farthest point
        let mut p = v[0];
        for i in 1..n {
            if dist(&c, &v[i]) > dist(&c, &p) {
                p = v[i];
            }
        }

        // print the answer
        if i == ITER {
            if c.x.abs() < 1e-9 { c.x = 0.0; }
            if c.y.abs() < 1e-9 { c.y = 0.0; }
            println!("{:.3} {:.3} {:.3}", c.x, c.y, dist(&c, &p));
            return;
        }

        // adjust the center
        c.x += (p.x - c.x) * r;
        c.y += (p.y - c.y) * r;
        r *= FAC;
    }

}
