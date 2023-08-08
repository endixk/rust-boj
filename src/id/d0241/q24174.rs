// BOJ 24174 [Heap Sort 2]
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

fn heapify(a: &mut [u32], i: usize, n: usize, c: &mut usize, k: usize) {
    let (l, r) = (i<<1, i<<1|1);
    let mut m = l;
    if r <= n {
        m = if a[l] < a[r] { l } else { r };
    } else if l > n {
        return;
    }

    if a[m] < a[i] {
        a.swap(m, i);
        *c += 1;
        if *c == k {
            let mut s = String::new();
            a.iter().skip(1).for_each(|&x| s += &format!("{} ", x));
            println!("{}", s.trim_end());
            std::process::exit(0);
        }
        heapify(a, m, n, c, k);
    }
}
fn build(a: &mut [u32], n: usize, c: &mut usize, k: usize) {
    for i in (1..=n/2).rev() {
        heapify(a, i, n, c, k);
    }
}
fn sort(a: &mut [u32], n: usize, c: &mut usize, k: usize) {
    build(a, n, c, k);
    for i in (2..=n).rev() {
        a.swap(1, i);
        *c += 1;
        if *c == k {
            let mut s = String::new();
            a.iter().skip(1).for_each(|&x| s += &format!("{} ", x));
            println!("{}", s.trim_end());
            std::process::exit(0);
        }
        heapify(a, 1, i-1, c, k);
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = vec![0; n+1];
    (1..=n).for_each(|i| a[i] = next::<u32>(&mut it));
    let mut c = 0;
    sort(&mut a, n, &mut c, k);
    println!("-1");
}
