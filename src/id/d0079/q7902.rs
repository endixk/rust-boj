// TODO BOJ 7902 [The Picnic]
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

#[derive(Eq, PartialEq, Clone, Copy, Debug)] struct Point { x: i32, y: i32, id: usize }
#[inline] fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64;
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}
#[inline] fn dist(a: &Point, b: &Point) -> i64 {
    ((a.x - b.x) as i64).pow(2) + ((a.y - b.y) as i64).pow(2)
}
#[inline] fn inside(a: &Point, b: &Point, c: &Point, p: &Point) -> bool {
    if ccw(a, b, c) < 0 { return inside(a, c, b, p); }
    ccw(a, b, p) > 0 && ccw(b, c, p) > 0 && ccw(c, a, p) > 0
}
#[inline] fn empty(points: &Vec<Point>, i: usize, j: usize, k: usize) -> bool {
    for x in 0..points.len() {
        if x == i || x == j || x == k { continue; }
        if inside(&points[i], &points[j], &points[k], &points[x]) { return false; }
    }
    true
}
#[inline] fn area(a: &Point, b: &Point, c: &Point) -> f64 {
    let x = (b.x - a.x) as f64 * (c.y - a.y) as f64 - (b.y - a.y) as f64 * (c.x - a.x) as f64;
    x.abs() / 2.0
}
fn area_poly(poly: &Vec<Point>) -> f64 {
    let mut ret = 0.0;
    for i in 1..poly.len()-1 {
        ret += area(&poly[0], &poly[i], &poly[i+1]);
    }
    ret
}
fn sweep(p: &Point, rp: &Vec<Point>, emp: &Vec<Vec<Vec<bool>>>, ans: &mut f64) {
    let mut polys = vec![(p, &rp[0], 0.0)];
    for i in 1..rp.len() {
        for j in 0..polys.len() {
            if ccw(polys[j].0, polys[j].1, &rp[i]) <= 0 { continue; }
            if !emp[polys[j].0.id][polys[j].1.id][rp[i].id] { continue; }
            polys.push((polys[j].1, &rp[i], polys[j].2 + area(polys[j].0, polys[j].1, &rp[i])));
        }
        polys.push((p, &rp[i], 0.0));
    }
    for poly in polys {
        *ans = ans.max(poly.2);
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let n = next::<usize>(&mut it);
        let mut points = vec![];
        for i in 0..n {
            points.push(Point { x: next(&mut it), y: next(&mut it), id: i });
        }

        let mut emp = vec![vec![vec![false; n]; n]; n];
        for i in 0..n-2 {
            for j in i+1..n-1 {
                for k in j+1..n {
                    let x = empty(&points, i, j, k);
                    emp[i][j][k] = x; emp[i][k][j] = x;
                    emp[j][i][k] = x; emp[j][k][i] = x;
                    emp[k][i][j] = x; emp[k][j][i] = x;
                }
            }
        }

        let mut ans = 0.0;
        for p in 0..n {
            let mut rp = vec![];
            for i in 0..n {
                if i == p { continue; }
                if points[p].x < points[i].x || (points[p].x == points[i].x && points[p].y < points[i].y) {
                    rp.push(points[i]);
                }
            }
            if rp.len() < 2 { continue; }

            rp.push(Point{ x: points[p].x - 1, y: points[p].y, id: n });
            rp.sort_by(|a, b| {
                let t = ccw(&points[p], a, b);
                if t == 0 {
                    dist(&points[p], b).cmp(&dist(&points[p], a))
                } else if t > 0 { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
            });
            let x = rp.iter().position(|x| x.id == n).unwrap();
            rp.rotate_left(x+1);
            rp.pop();

            sweep(&points[p], &rp, &emp, &mut ans);
        }

        writeln!(so, "{:.1}", ans)?;
    }

    Ok(())
}
