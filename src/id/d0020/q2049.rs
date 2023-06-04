// BOJ 2049 [Farthest Points]
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

fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64;
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
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

fn dist(a: &Point, b: &Point) -> i64 {
    ((a.x - b.x) as i64).pow(2) + ((a.y - b.y) as i64).pow(2)
}
fn cross(x1: &Point, x2: &Point, y1: &Point, y2: &Point) -> i64 {
    (x2.x - x1.x) as i64 * (y2.y - y1.y) as i64 - (x2.y - x1.y) as i64 * (y2.x - y1.x) as i64
}
fn calipers(hull: Vec<Point>) -> (Point, Point) {
    // find the leftmost and rightmost points
    let (mut l, mut r) = (0, 0);
    hull.iter().enumerate().skip(1).for_each(|(i, p)| {
        if p.x < hull[l].x { l = i; }
        if p.x > hull[r].x { r = i; }
    });

    // find the farthest distance
    let mut d = dist(&hull[l], &hull[r]);
    let mut ret = (hull[l].clone(), hull[r].clone());
    for _ in 1..hull.len() {
        // rotate
        if cross(&hull[l], &hull[(l + 1) % hull.len()], &hull[r], &hull[(r + 1) % hull.len()]) < 0 {
            l = (l + 1) % hull.len();
        } else {
            r = (r + 1) % hull.len();
        }
        // update
        let t = dist(&hull[l], &hull[r]);
        if t > d {
            d = t;
            ret = (hull[l].clone(), hull[r].clone());
        }
    }

    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let points = (0..n).map(|_| {
        let x = next::<i32>(&mut it);
        let y = next::<i32>(&mut it);
        Point { x, y }
    }).collect::<Vec<_>>();

    let (p, q) = calipers(graham(points));
    println!("{}", dist(&p, &q));
}
