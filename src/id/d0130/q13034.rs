// BOJ 13034 [Polygon Game]
// Supported by GitHub Copilot

use std::io::{self, BufRead};
fn mex(mut v: Vec<i32>) -> i32 {
    v.sort_unstable_by(|a, b| b.cmp(a));
    v.dedup();

    let mut x = 0;
    while let Some(k) = v.pop() {
        if k == x { x += 1; }
        else { return x; }
    }
    x
}
pub fn main() {
    let n = io::stdin().lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut g = vec![0; n+1];
    g[2] = 1;
    for i in 3..=n {
        let mut v = Vec::new();
        for j in 0..i/2 {
            v.push(g[j] ^ g[i-j-2]);
        }
        g[i] = mex(v);
    }
    println!("{}", if g[n] > 0 { 1 } else { 2 });
}
