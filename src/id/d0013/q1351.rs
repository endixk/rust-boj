// BOJ 1351 [Infinite Sequence]
// Supported by GitHub Copilot

use std::io::{self, BufRead};
use std::collections::HashMap;

fn go(map: &mut HashMap<i64, i64>, x: i64, p: i64, q: i64) -> i64 {
    if x == 0 { return 1; }
    if map.contains_key(&x) { return map[&x]; }
    let res = go(map, x / p, p, q) + go(map, x / q, p, q);
    map.insert(x, res);
    res
}
pub fn main() {
    let v = io::stdin().lock().lines().next().unwrap().unwrap()
        .split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let (n, p, q) = (v[0], v[1], v[2]);
    let mut map = HashMap::new();
    println!("{}", go(&mut map, n, p, q));
}
