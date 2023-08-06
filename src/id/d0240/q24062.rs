// BOJ 24062 [Merge Sort 3]
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

fn merge(a: &mut [i64], b: &[i64], p: usize, q: usize, r: usize, d: &mut i64) {
    let mut tmp = vec![0; r-p+1];
    let (mut i, mut j) = (p, q+1);

    let mut x = 0;
    while i <= q && j <= r {
        if a[i] <= a[j] { tmp[x] = a[i]; i += 1; }
        else { tmp[x] = a[j]; j += 1; }
        x += 1;
    }
    while i <= q { tmp[x] = a[i]; i += 1; x += 1; }
    while j <= r { tmp[x] = a[j]; j += 1; x += 1; }
    for x in 0..r-p+1 {
        *d += (tmp[x] - b[p+x]).abs() - (a[p+x] - b[p+x]).abs();
        if *d == 0 {
            println!("1");
            std::process::exit(0);
        }
        a[p+x] = tmp[x];
    }
}
fn sort(a: &mut [i64], b: &[i64], p: usize, r: usize, d: &mut i64) {
    if p < r {
        let q = (p+r)>>1;
        sort(a, b, p, q, d);
        sort(a, b, q+1, r, d);
        merge(a, b, p, q, r, d);
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut a = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let b = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let mut d = (0..n).fold(0, |acc, i| acc + (a[i] - b[i]).abs());
    if d == 0 {
        println!("1");
        return;
    }
    sort(&mut a, &b, 0, n-1, &mut d);
    println!("0");
}
