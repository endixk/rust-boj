// BOJ 9527 [Counting Ones]
// Supported by GitHub Copilot

use std::io::BufRead;

fn solve(a: u64, b: u64, p: usize, f: &Vec<u64>) -> u64 {
    if p == 0 { return b }
    let x = 1<<p;
    if a == 0 && b == (x<<1)-1 { return f[p] }
    if b < x { return solve(a, b, p-1, f) }
    if a >= x { return solve(a-x, b-x, p-1, f) + b - a + 1 }
    solve(a, x-1, p-1, f) + solve(0, b-x, p-1, f) + b - x + 1
}

pub fn main() {
    let s = std::io::stdin().lock().lines().next().unwrap().unwrap();
    let v = s.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut f = vec![0u64; 53];
    f[0] = 1;
    for p in 1..53 {
        f[p] = (f[p-1] << 1) + (1 << p);
    }
    println!("{}", solve(v[0], v[1], 53, &f));
}
