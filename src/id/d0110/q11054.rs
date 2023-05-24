// BOJ 11054 [Longest Bitonic Subsequence]
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

fn lower_bound(arr: &[i32], val: i32) -> usize {
    let mut l = 0;
    let mut r = arr.len();
    while l < r {
        let m = (l + r) / 2;
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
    let mut arr = (0..n).map(|_| next(&mut it)).collect::<Vec<i32>>();

    let mut lis: Vec<i32> = Vec::with_capacity(n);
    let mut lln: Vec<usize> = Vec::with_capacity(n);
    for a in arr.iter() {
        if lis.is_empty() || lis.last().unwrap() < a {
            lis.push(*a);
            lln.push(lis.len())
        } else {
            let i = lower_bound(&lis, *a);
            lis[i] = *a;
            lln.push(i+1);
        }
    }

    arr.reverse();
    lis.clear();
    let mut ans = 0;
    for a in arr.iter() {
        if lis.is_empty() || lis.last().unwrap() < a {
            lis.push(*a);
            ans = ans.max(lln.pop().unwrap() + lis.len() - 1);
        } else {
            let i = lower_bound(&lis, *a);
            lis[i] = *a;
            ans = ans.max(lln.pop().unwrap() + i);
        }
    }

    writeln!(so, "{}", ans).unwrap();
}
