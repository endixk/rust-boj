// BOJ 2561 [Flip Thrice]
// Supported by GitHub Copilot

use std::io::{self, Read};

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn check(v: &Vec<i16>) -> bool {
    let mut x = 1;
    for &k in v {
        if k < 0 { continue; }
        if k != x { return false; }
        x += 1;
    }
    true
}

fn flip(v: &Vec<i16>, log: &Vec<usize>, d: u8) {
    if check(&v) {
        log.iter().for_each(|&x| print!("{} ", x));
        std::process::exit(0);
    }
    if d == 3 { return; }

    let locs = v.iter().enumerate().filter(|&(_, &x)| x < 0).map(|(i, _)| i).collect::<Vec<usize>>();
    for i in 0..locs.len()-1 { for j in i+1..locs.len() {
        let mut next = vec![];
        for k in 0..locs[i] { next.push(v[k]); }
        for k in locs[i]..=locs[j] { next.push(v[locs[i]+locs[j]-k]); }
        for k in locs[j]+1..v.len() { next.push(v[k]); }
        let mut next_log = log.clone();
        next_log[(d<<1) as usize] = locs[i]-i+1;
        next_log[(d<<1) as usize+1] = locs[j]-j;
        flip(&next, &next_log, d+1);
    }}
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let a = (0..n).map(|_| next::<i16>(&mut it)).collect::<Vec<i16>>();
    let mut v = vec![-1];
    let mut i = 0;
    loop {
        let mut w = vec![];
        v.push(a[i]);
        while i < n-1 && (a[i] - a[i+1]).abs() == 1 { w.push(a[i+1]); i += 1; }
        if w.len() == 1 { v.push(-1); v.push(w[0]); }
        else {
            for x in w { v.push(x); }
        }
        v.push(-1);
        i += 1;
        if i == n { break; }
    }

    flip(&v, &vec![1, 1, 1, 1, 1, 1], 0);
}