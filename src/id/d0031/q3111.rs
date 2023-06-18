// BOJ 3111 [CENZURA]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

pub fn main() {
    let p = io::stdin().lock().lines().next().unwrap().unwrap();
    let q = p.chars().rev().collect::<String>();
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let s = s.as_bytes();

    let (mut l, mut r) = (0, s.len() - 1);
    let mut flag = true;
    let (mut ls, mut rs) = (String::new(), String::new());
    while l <= r {
        if flag {
            ls.push(s[l] as char);
            if ls.ends_with(&p) {
                ls.truncate(ls.len() - p.len());
                flag = false;
            }
            l += 1;
        } else {
            rs.push(s[r] as char);
            if rs.ends_with(&q) {
                rs.truncate(rs.len() - q.len());
                flag = true;
            }
            r -= 1;
        }
    }

    let mut ans = ls + &rs.chars().rev().collect::<String>();
    while ans.contains(&p) {
        ans = ans.replace(&p, "");
    }
    println!("{}", ans);
}
