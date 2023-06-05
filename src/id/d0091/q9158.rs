// BOJ 9158 [Super Star]
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
    z: f64,
}
fn dist(a: &Point, b: &Point) -> f64 {
    (a.x - b.x).hypot(a.y - b.y).hypot(a.z - b.z)
}

const ITER: usize = 10000;
const RTO: f64 = 0.1; // initial ratio
const FAC: f64 = 0.995; // ratio factor
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    loop {
        let n = next::<usize>(&mut it);
        if n == 0 { break; }
        let v = (0..n).map(|_| {
            let x = next::<f64>(&mut it);
            let y = next::<f64>(&mut it);
            let z = next::<f64>(&mut it);
            Point { x, y, z }
        }).collect::<Vec<_>>();

        let mut c = Point {
            x: v.iter().map(|p| p.x).sum::<f64>() / n as f64,
            y: v.iter().map(|p| p.y).sum::<f64>() / n as f64,
            z: v.iter().map(|p| p.z).sum::<f64>() / n as f64,
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
                println!("{:.5}", dist(&c, &p));
                break;
            }

            // adjust the center
            c.x += (p.x - c.x) * r;
            c.y += (p.y - c.y) * r;
            c.z += (p.z - c.z) * r;
            r *= FAC;
        }
    }

}
