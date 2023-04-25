// BOJ 11444 [Fibonacci Number 6]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

struct Mat {
    x00: i64,
    x01: i64,
    x10: i64,
    x11: i64,
}

fn mat_mul(a: &Mat, b: &Mat, m: i64) -> Mat {
    Mat {
        x00: (a.x00 * b.x00 + a.x01 * b.x10) % m,
        x01: (a.x00 * b.x01 + a.x01 * b.x11) % m,
        x10: (a.x10 * b.x00 + a.x11 * b.x10) % m,
        x11: (a.x10 * b.x01 + a.x11 * b.x11) % m,
    }
}

fn pow(a: &Mat, p: i64, m: i64) -> Mat {
    if p == 1 {
        return Mat{x00: a.x00, x01: a.x01, x10: a.x10, x11: a.x11};
    }
    let b = pow(a, p / 2, m);
    let c = mat_mul(&b, &b, m);
    if p % 2 == 0 {
        c
    } else {
        mat_mul(&c, a, m)
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    writeln!(so, "{}",
             pow(
                 &Mat{x00: 1, x01: 1, x10: 1, x11: 0},
                 read(&mut si).trim().parse().unwrap(),
                 1_000_000_007).x10).unwrap();
}
