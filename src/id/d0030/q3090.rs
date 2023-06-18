// BOJ 3090 [ŽIVICA]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};

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

fn cost(a: &Vec<i32>, n: usize, d: i32) -> (Vec<i32>, u64) {
    let mut ret = 0;
    let mut a = a.clone();
    for i in 1..n {
        if a[i] > a[i-1] + d {
            ret += (a[i] - (a[i-1] + d)) as u64;
            a[i] = a[i-1] + d;
        }
    }
    for i in (1..n).rev() {
        if a[i-1] > a[i] + d {
            ret += (a[i-1] - (a[i] + d)) as u64;
            a[i-1] = a[i] + d;
        }
    }
    (a, ret)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, t) = (next::<usize>(&mut it), next::<u64>(&mut it));
    let a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();

    let (mut l, mut r) = (0, 1_000_000_000);
    let mut ans = vec![0; n];
    while l <= r {
        let mid = (l + r) / 2;
        let (a, c) = cost(&a, n, mid);
        if c <= t {
            ans = a;
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    ans.iter().for_each(|&x| write!(so, "{} ", x).unwrap());
}
