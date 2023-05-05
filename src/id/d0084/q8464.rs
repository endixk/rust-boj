// BOJ 8464 [Non-Squarefree Numbers]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn square_free(mo: &Vec<i8>, k: u64, sz: usize) -> u64 {
    let mut ret = 0;
    for i in 2..sz {
        match mo[i] {
            1 => ret -= k / (i as u64 * i as u64),
            -1 => ret += k / (i as u64 * i as u64),
            _ => continue,
        }
    }
    ret
}

pub fn main() {
    let k = io::stdin().lock().lines().next().unwrap().unwrap().parse::<u64>().unwrap();

    let aprx = (k as f64 * 2.5505460967304304402864869634761) as u64;
    let sz = (aprx as f64).sqrt() as usize + 10;
    let mut mo = vec![0i8; sz];
    mo[1] = 1;
    for i in 1..sz {
        for j in (i<<1..sz).step_by(i) {
            mo[j] -= mo[i];
        }
    }

    let (mut lo, mut hi) = (if aprx > 600 { aprx - 600 } else { 1 }, aprx + 600);
    while lo < hi {
        let mid = (lo + hi) >> 1;
        if square_free(&mo, mid, sz) < k {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    println!("{}", lo);
}