// BOJ 14003 [Longest Increasing Subsequence 5]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
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

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut arr: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(it.next().unwrap().parse().unwrap());
    }

    let mut lis: Vec<i32> = Vec::with_capacity(n);
    let mut lix: Vec<usize> = Vec::with_capacity(n);
    for &a in arr.iter() {
        if lis.is_empty() || lis.last().unwrap() < &a {
            lis.push(a);
            lix.push(lis.len() - 1);
        } else {
            let i = lower_bound(&lis, a);
            lis[i] = a;
            lix.push(i);
        }
    }

    let cnt = lis.len();
    let mut ans: Vec<i32> = Vec::with_capacity(cnt);
    let mut i: isize = cnt as isize - 1;
    for j in (0..n).rev() {
        if lix[j] == i as usize {
            ans.push(arr[j]);
            i -= 1;
        }
    }

    writeln!(so, "{}", cnt).unwrap();
    for a in ans.iter().rev() {
        write!(so, "{} ", a).unwrap();
    }
}
