// BOJ 24061 [Merge Sort 2]
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

fn merge(a: &mut [u32], p: usize, q: usize, r: usize, c: &mut usize, k: usize, so: &mut dyn Write) {
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
        a[p+x] = tmp[x];
        *c += 1;
        if *c == k {
            a.iter().for_each(|x| write!(so, "{} ", x).unwrap());
            so.flush().ok();
            std::process::exit(0);
        }
    }
}
fn sort(a: &mut [u32], p: usize, r: usize, c: &mut usize, k: usize, so: &mut dyn Write) {
    if p < r {
        let q = (p+r)>>1;
        sort(a, p, q, c, k, so);
        sort(a, q+1, r, c, k, so);
        merge(a, p, q, r, c, k, so);
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = (0..n).map(|_| next::<u32>(&mut it)).collect::<Vec<_>>();
    let mut c = 0;
    sort(&mut a, 0, n-1, &mut c, k, &mut so);
    println!("-1");
}
