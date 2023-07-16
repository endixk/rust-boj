use std::io::{self, BufRead};
pub fn main() {
    let mut h = io::stdin().lock().lines().map(|x| x.unwrap().parse::<usize>().unwrap()).collect::<Vec<_>>();
    h.sort_unstable();
    let s = h.iter().sum::<usize>() - 100;
    for i in 0..8 { for j in i+1..9 {
        if s == h[i] + h[j] {
            for k in 0..9 {
                if k != i && k != j { println!("{}", h[k]); }
            }
            return;
        }
    }}
}
