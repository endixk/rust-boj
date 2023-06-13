// BOJ 9373 [Getting Through]
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

fn get(x: &Vec<f64>, y: &Vec<f64>, r: &Vec<f64>, i: usize, j: usize, w: f64, n: usize) -> f64 {
    if i == n { return w }

    let (x1, y1, r1) = (x[i], y[i], r[i]);
    if j == n+1 {
        return if w > x1 + r1 { w - x1 - r1 } else { 0.0 }
    }
    if j == n {
        return if x1 > r1 { x1 - r1 } else { 0.0 }
    }

    let (x2, y2, r2) = (x[j], y[j], r[j]);
    let d = (x1 - x2).hypot(y1 - y2);
    return if d > r1 + r2 { d - r1 - r2 } else { 0.0 }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let (w, n) = (next::<f64>(&mut it), next::<usize>(&mut it));
        let (mut x, mut y, mut r) = (
            Vec::with_capacity(n),
            Vec::with_capacity(n),
            Vec::with_capacity(n));
        for _ in 0..n {
            x.push(next::<f64>(&mut it));
            y.push(next::<f64>(&mut it));
            r.push(next::<f64>(&mut it));
        }

        const INF: f64 = 1e9;
        let mut ms = 0.0;
        let mut vis = vec![false; n+2];
        let mut dist = vec![INF; n+2];
        dist[n] = 0.0;
        for _ in 0..n+2 {
            let mut min = INF;
            let mut u = 0;
            for i in 0..n+2 {
                if !vis[i] && dist[i] < min {
                    min = dist[i];
                    u = i;
                }
            }
            if ms < min { ms = min; }
            vis[u] = true;
            if u == n+1 {
                writeln!(so, "{:.6}", ms / 2.0).ok();
                break;
            }

            for v in 0..n+2 {
                let adj = if u < v { get(&x, &y, &r, u, v, w, n) } else { get(&x, &y, &r, v, u, w, n) };
                if !vis[v] && adj < dist[v] {
                    dist[v] = adj;
                }
            }
        }
    }
}
