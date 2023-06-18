// BOJ 16890 [Start-Up]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let t = io::stdin().lock().lines().next().unwrap().unwrap();

    let mut s = s.chars().collect::<Vec<_>>();
    let mut t = t.chars().collect::<Vec<_>>();
    s.sort_unstable();
    t.sort_unstable_by(|a, b| b.cmp(a));

    let mut ans = vec![' '; s.len()];
    let mut turn = true;
    let (mut ip, mut iq) = (0, (s.len()+1)/2);
    let (mut jp, mut jq) = (0, s.len()/2);
    let (mut p, mut q) = (0, s.len());
    for _ in 0..s.len() {
        if turn {
            if s[ip] < t[jp] {
                ans[p] = s[ip]; ip += 1; p += 1;
            } else {
                ans[q-1] = s[iq-1]; iq -= 1; q -= 1;
            }
        } else {
            if s[ip] < t[jp] {
                ans[p] = t[jp]; jp += 1; p += 1;
            } else {
                ans[q-1] = t[jq-1]; jq -= 1; q -= 1;
            }
        }
        turn = !turn;
    }

    println!("{}", ans.iter().collect::<String>());
}
