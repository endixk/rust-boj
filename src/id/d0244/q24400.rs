// BOJ 24400 [Selection Algorithm 3]
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

fn partition(a: &mut Vec<i64>, p: usize, r: usize, b: &[i64], d: &mut i64) -> usize {
    let (x, mut i) = (a[r], p-1);
    for j in p..r {
        if a[j] <= x {
            i += 1;
            *d += (a[j] - b[i]).abs() + (a[i] - b[j]).abs() - (a[j] - b[j]).abs() - (a[i] - b[i]).abs();
            if *d == 0 { println!("1"); std::process::exit(0); }
            a.swap(i, j);
        }
    }
    if i+1 != r {
        *d += (a[i+1] - b[r]).abs() + (a[r] - b[i+1]).abs() - (a[i+1] - b[i+1]).abs() - (a[r] - b[r]).abs();
        if *d == 0 { println!("1"); std::process::exit(0); }
        a.swap(i+1, r);
    }
    i+1
}

fn select(a: &mut Vec<i64>, p: usize, r: usize, q: usize, b: &[i64], d: &mut i64) {
    if p == r { return; }
    let t = partition(a, p, r, b, d);
    let x = t-p+1;
    if q < x { select(a, p, t-1, q, b, d) }
    else if q > x { select(a, t+1, r, q-x, b, d) }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = vec![0; n+1];
    for i in 1..=n { a[i] = next::<i64>(&mut it); }
    let mut b = vec![0; n+1];
    let mut d = 0;
    for i in 1..=n {
        b[i] = next::<i64>(&mut it);
        d += (a[i] - b[i]).abs();
    }
    if d == 0 { println!("1"); return Ok(()); }
    
    select(&mut a, 1, n, q, &b, &mut d);
    println!("0");

    Ok(())
}
