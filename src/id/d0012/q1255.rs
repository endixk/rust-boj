// BOJ 1255 [The War]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::cmp::{Ordering, min, max, Reverse};
use std::collections::BinaryHeap;

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

struct Point { x: i64, y: i64 }
impl Eq for Point {}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x == other.x {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Clone for Point {
    fn clone(&self) -> Self {
        Point { x: self.x, y: self.y }
    }
}

fn dist(a: &Point, b: &Point) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    (dx * dx + dy * dy).sqrt()
}
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

struct SF64(f64);
impl Eq for SF64 {}
impl PartialEq for SF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Ord for SF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 { Ordering::Less }
        else if self.0 > other.0 { Ordering::Greater }
        else { Ordering::Equal }
    }
}
impl PartialOrd for SF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Clone for SF64 {
    fn clone(&self) -> Self {
        SF64(self.0)
    }
}
impl Copy for SF64 {}

fn dijkstra(adj: &Vec<Vec<(usize, SF64)>>, dist: &mut Vec<SF64>, src: usize) {
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(SF64(0.0)), src));
    dist[src] = SF64(0.0);
    while let Some((Reverse(d), u)) = pq.pop() {
        if dist[u] < d { continue; }
        for (v, w) in &adj[u] {
            let next = SF64(dist[u].0 + w.0);
            if dist[*v] > next {
                dist[*v] = next;
                pq.push((Reverse(dist[*v]), *v));
            }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let civs = (0..n).map(|_| {
        let (co, v) = (next::<String>(&mut it), next::<i64>(&mut it));
        let (x, y) = co[1..co.len()-1].split_at(co.find(',').unwrap());
        (Point { x: x[..x.len()-1].parse().unwrap(), y: y.parse().unwrap() }, v)
    }).collect::<Vec<_>>();

    let bars = (0..m).map(|_| {
        let (cx, _, cy) = (next::<String>(&mut it), next::<String>(&mut it), next::<String>(&mut it));
        let (x1, y1) = cx[1..cx.len()-1].split_at(cx.find(',').unwrap());
        let (x2, y2) = cy[1..cy.len()-1].split_at(cy.find(',').unwrap());
        Line { a: Point { x: x1[..x1.len()-1].parse().unwrap(), y: y1.parse().unwrap() },
                b: Point { x: x2[..x2.len()-1].parse().unwrap(), y: y2.parse().unwrap() } }
    }).collect::<Vec<_>>();

    // civs: 1..n, bars: (n+1, n+m+1)..(n+m, n+m+m+1), sink: 0
    let mut adj = vec![vec![]; n+m*2+2];

    // check civilians
    for (i, civ) in civs.iter().enumerate() {
        // check access to sink
        let l = Line { a: civ.0.clone(), b: Point { x: civ.0.x, y: 0 } };
        let mut ok = true;
        for bar in bars.iter() {
            if intersect(&l, bar) {
                ok = false;
                break;
            }
        }
        if ok { adj[i+1].push((0, SF64(civ.0.y as f64))); }

        // check access to bars
        for (j, bar) in bars.iter().enumerate() {
            let l = Line { a: civ.0.clone(), b: bar.a.clone() };
            ok = true;
            for (k, other) in bars.iter().enumerate() {
                if j == k { continue; }
                if intersect(&l, other) {
                    ok = false;
                    break;
                }
            }
            if ok { adj[i+1].push((n+j+1, SF64(dist(&civ.0, &bar.a)))); }

            let l = Line { a: civ.0.clone(), b: bar.b.clone() };
            ok = true;
            for (k, other) in bars.iter().enumerate() {
                if j == k { continue; }
                if intersect(&l, other) {
                    ok = false;
                    break;
                }
            }
            if ok { adj[i+1].push((n+j+m+1, SF64(dist(&civ.0, &bar.b)))); }
        }
    }

    // check bars
    for (i, bar) in bars.iter().enumerate() {
        // check access to sink of bar.a
        let l = Line { a: bar.a.clone(), b: Point { x: bar.a.x, y: 0 } };
        let mut ok = true;
        for (j, other) in bars.iter().enumerate() {
            if i == j { continue; }
            if intersect(&l, other) {
                ok = false;
                break;
            }
        }
        if ok { adj[n+i+1].push((0, SF64(bar.a.y as f64))); }

        // check access to sink of bar.b
        let l = Line { a: bar.b.clone(), b: Point { x: bar.b.x, y: 0 } };
        ok = true;
        for (j, other) in bars.iter().enumerate() {
            if i == j { continue; }
            if intersect(&l, other) {
                ok = false;
                break;
            }
        }
        if ok { adj[n+i+m+1].push((0, SF64(bar.b.y as f64))); }

        for (j, other) in bars.iter().enumerate() {
            if i == j { continue; }

            // check access of bar.a to other bars
            let l = Line { a: bar.a.clone(), b: other.a.clone() };
            ok = true;
            for (k, other2) in bars.iter().enumerate() {
                if i == k || j == k { continue; }
                if intersect(&l, other2) {
                    ok = false;
                    break;
                }
            }
            if ok { adj[n+i+1].push((n+j+1, SF64(dist(&bar.a, &other.a)))); }

            let l = Line { a: bar.a.clone(), b: other.b.clone() };
            ok = true;
            for (k, other2) in bars.iter().enumerate() {
                if i == k || j == k { continue; }
                if intersect(&l, other2) {
                    ok = false;
                    break;
                }
            }
            if ok { adj[n+i+1].push((n+j+m+1, SF64(dist(&bar.a, &other.b)))); }

            // check access of bar.b to other bars
            let l = Line { a: bar.b.clone(), b: other.a.clone() };
            ok = true;
            for (k, other2) in bars.iter().enumerate() {
                if i == k || j == k { continue; }
                if intersect(&l, other2) {
                    ok = false;
                    break;
                }
            }
            if ok { adj[n+i+m+1].push((n+j+1, SF64(dist(&bar.b, &other.a)))); }

            let l = Line { a: bar.b.clone(), b: other.b.clone() };
            ok = true;
            for (k, other2) in bars.iter().enumerate() {
                if i == k || j == k { continue; }
                if intersect(&l, other2) {
                    ok = false;
                    break;
                }
            }
            if ok { adj[n+i+m+1].push((n+j+m+1, SF64(dist(&bar.b, &other.b)))); }
        }
    }

    let mut ans = 0.0;
    for (i, civ) in civs.iter().enumerate() {
        let mut d = vec![SF64(1e100); n+m*2+2];
        dijkstra(&adj, &mut d, i+1);
        if d[0].0 / civ.1 as f64 > ans { ans = d[0].0 / civ.1 as f64; }
    }
    println!("{:.1}", ans);
}
