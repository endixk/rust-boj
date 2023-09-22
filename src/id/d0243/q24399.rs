// BOJ 24399 [Selection Algorithm 2]
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

fn report(a: &[u32]) -> ! {
    a.iter().skip(1).for_each(|x| print!("{} ", x));
    std::process::exit(0);
}

fn partition(a: &mut Vec<u32>, p: usize, r: usize, c: &mut usize, k: usize) -> usize {
    let (x, mut i) = (a[r], p-1);
    for j in p..r {
        if a[j] <= x {
            i += 1;
            a.swap(i, j);
            *c += 1;
            if *c == k { report(a); }
        }
    }
    if i+1 != r {
        a.swap(i+1, r);
        *c += 1;
        if *c == k { report(a); }
    }
    i+1
}

fn select(a: &mut Vec<u32>, p: usize, r: usize, q: usize, c: &mut usize, k: usize) {
    if p == r { return; }
    let t = partition(a, p, r, c, k);
    let x = t-p+1;
    if q < x { select(a, p, t-1, q, c, k) }
    else if q > x { select(a, t+1, r, q-x, c, k) }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, q, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = vec![0; n+1];
    for i in 1..=n {
        a[i] = next::<u32>(&mut it);
    }
    let mut c = 0;
    select(&mut a, 1, n, q, &mut c, k);
    println!("-1");

    Ok(())
}
