// BOJ 14583 [이음줄]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut it = s.split_ascii_whitespace();
    let (h, v) = (it.next().unwrap().parse::<f64>().unwrap(), it.next().unwrap().parse::<f64>().unwrap());
    let x = (h * v.hypot(h) - h.powi(2)) / v;
    println!("{:.2} {:.2}", h.hypot(x) / 2.0, (v - x) * ((1.0 + h / v.hypot(h)) / 2.0).sqrt());
}
