// BOJ 18880 [Social Distancing 1]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

fn place_one(size: u32, edge: bool) -> u32 {
    if edge { size }
    else { (size + 1) >> 1 }
}

fn place_two(size: u32, edge: bool) -> u32 {
    if edge { place_one(size - 1, false) }
    else { (size + 1) / 3 }
}

pub fn main() {
    io::stdin().lock().lines().next().unwrap().unwrap();
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let v = s.split("1").map(|x| x.len() as u32).filter(|&x| x > 0).collect::<Vec<u32>>();

    let s = s.as_bytes();
    if v.len() == 1 && s[0] == b'0' && s[s.len() - 1] == b'0' {
        println!("{}", v[0] - 1);
        return;
    }

    let mut e = vec![false; v.len()];
    e[0] = s[0] == b'0';
    e[v.len() - 1] = s[s.len() - 1] == b'0';

    if v.len() == 1 {
        println!("{}", place_two(v[0], e[0]));
        return;
    }

    let mut v = v.iter().zip(e.iter())
        .map(|(&x, &y)| (place_one(x, y), x, y)).collect::<Vec<(u32, u32, bool)>>();
    v.sort_unstable();
    v.reverse();

    // split more
    let mut two = place_two(v[0].1, v[0].2);
    for i in 1..v.len() {
        if !v[i].2 {
            two = two.min(v[i].1 + 1);
        }
    }

    // split other
    let mut one = v[1].0;
    for i in 2..v.len() {
        if !v[i].2 {
            one = one.min(v[i].1 + 1);
        }
    }

    println!("{}", one.max(two));
}
