// BOJ 3408 [Non-Boring Sequences]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn unique_element(i: i32, s: i32, e: i32, ploc: &Vec<i32>, nloc: &Vec<i32>) -> bool {
    ploc[i as usize] < s && nloc[i as usize] >= e
}

fn non_boring(s: i32, e: i32, ploc: &Vec<i32>, nloc: &Vec<i32>) -> bool {
    if e-s < 2 { return true; }
    for i in 1..=(e-s)/2 {
        if unique_element(s+i, s, e, ploc, nloc) {
            return non_boring(s, s + i, ploc, nloc) && non_boring(s + i, e, ploc, nloc);
        } else if unique_element(e-i, s, e, ploc, nloc) {
            return non_boring(s, e - i, ploc, nloc) && non_boring(e - i, e, ploc, nloc);
        }
    }

    false
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    for _ in 0..next(&mut it) {
        let n = next(&mut it);

        let mut map = HashMap::<i32, usize>::new();
        let mut ploc = vec![0; n+2];
        let mut nloc = vec![1+n as i32; n+2];
        for i in 1..=n {
            let v = next::<i32>(&mut it);
            if let Some(&loc) = map.get(&v) {
                ploc[i] = loc as i32;
                nloc[loc] = i as i32;
                map.insert(v, i);
            } else {
                map.insert(v, i);
            }
        }

        writeln!(so, "{}", if non_boring(1, 1+n as i32, &ploc, &nloc) { "non-boring" } else { "boring" }).unwrap();
    }
}
