// BOJ 1353 [Sum and Product]
use std::f64::consts::E;
use std::io::BufRead;
fn calc(s: f64, x: f64) -> f64 {
    (s / x).powf(x)
}
pub fn main() {
    let it = std::io::stdin().lock().lines().next().unwrap().unwrap();
    let mut it = it.split_whitespace();
    let (s, p) = (it.next().unwrap().parse::<f64>().unwrap(), it.next().unwrap().parse::<f64>().unwrap());
    if s == p { println!("1"); return; }
    if p + 1e-6 > calc(s, (s/E).floor()) && p + 1e-6 > calc(s, (s/E).ceil()) { println!("-1"); return; }
    for x in 2.. {
        if p < calc(s, x as f64) + 1e-6 { println!("{}", x); return; }
    }
}