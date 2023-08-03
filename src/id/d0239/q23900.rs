// BOJ 23900 [Selection Sort 6]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = (0..n as u32).map(|i| (next::<u32>(&mut it), i)).collect::<Vec<_>>();
    let mut w = (0..n as u32).map(|i| (next::<u32>(&mut it), i)).collect::<Vec<_>>();
    if v == w {
        println!("1");
        return;
    }

    w.sort_unstable();
    let mut wval = vec![0; n];
    w.into_iter().enumerate().for_each(|(j, (_, i))| {
        wval[i as usize] = j;
    });

    v.sort_unstable();
    let mut val = vec![0; n];
    let mut loc = vec![0; n];
    v.into_iter().enumerate().for_each(|(j, (_, i))| {
        val[i as usize] = j;
        loc[j] = i as usize;
    });
    for i in (0..n).rev() {
        if loc[i] == i { continue; }
        loc[val[i]] = loc[i];
        val[loc[i]] = val[i];
        val[i] = i;
        if val == wval {
            println!("1");
            return;
        }
    }
    println!("0");
}
