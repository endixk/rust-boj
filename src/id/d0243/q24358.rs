// BOJ 24358 [Zayo Bayo]
use std::io::{stdin, BufRead};
pub fn main() {
    let v = stdin().lock().lines().next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut w = stdin().lock().lines().next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    w[0] += v[1] * (v[0] * 2 - 1) - 1;
    let c = match w[1] { 2 if w[2] % 4 == 0 && w[2] % 100 != 0 || w[2] % 400 == 0 => 29, 2 => 28, 4 | 6 | 9 | 11 => 30, _ => 31 };
    if w[0] > c { w[0] -= c; w[1] += 1; }
    if w[1] == 13 { w[1] = 1; w[2] += 1; }
    println!("{} {} {}", w[0], w[1], w[2]);
}