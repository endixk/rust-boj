// BOJ 2568 [Electric Wires 2]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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

fn lower_bound(arr: &[u32], val: u32) -> usize {
    let mut l = 0;
    let mut r = arr.len();
    while l < r {
        let m = l + r >> 1;
        if arr[m] < val {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = (0..n).map(|_|
        (next::<u32>(&mut it), next::<u32>(&mut it))).collect::<Vec<_>>();
    v.sort_unstable();

    let mut lis: Vec<u32> = Vec::with_capacity(n);
    let mut lix: Vec<usize> = Vec::with_capacity(n);
    for &(_, b) in v.iter() {
        if lis.is_empty() || lis.last().unwrap() < &b {
            lis.push(b);
            lix.push(lis.len() - 1);
        } else {
            let i = lower_bound(&lis, b);
            lis[i] = b;
            lix.push(i);
        }
    }

    let cnt = lis.len();
    let mut trk: Vec<bool> = vec![false; 500005];
    for i in 0..n {
        trk[v[i].0 as usize] = true;
    }
    let mut i: i32 = cnt as i32 - 1;
    for j in (0..n).rev() {
        if lix[j] == i as usize {
            trk[v[j].0 as usize] = false;
            i -= 1;
        }
    }

    writeln!(so, "{}", n - cnt).ok();
    for i in 0..500005 {
        if trk[i] {
            write!(so, "{} ", i).ok();
        }
    }
}
