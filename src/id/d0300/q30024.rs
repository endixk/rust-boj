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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec![vec![0; m+2]; n+2];
    let mut v = vec![vec![true; m+2]; n+2];
    for i in 1..=n { for j in 1..=m {
        b[i][j] = next::<i32>(&mut it);
        v[i][j] = false;
    }}

    let mut pq = std::collections::BinaryHeap::new();
    for i in 1..=n {
        pq.push((b[i][1], i, 1)); v[i][1] = true;
        if m > 1 {
            pq.push((b[i][m], i, m));
            v[i][m] = true;
        }
    }
    for j in 2..m {
        pq.push((b[1][j], 1, j)); v[1][j] = true;
        if n > 1 {
            pq.push((b[n][j], n, j));
            v[n][j] = true;
        }
    }

    for _ in 0..next::<usize>(&mut it) {
        let (_, i, j) = pq.pop().unwrap();
        writeln!(so, "{} {}", i, j)?;
        if !v[i-1][j] {
            pq.push((b[i-1][j], i-1, j)); v[i-1][j] = true;
        }
        if !v[i+1][j] {
            pq.push((b[i+1][j], i+1, j)); v[i+1][j] = true;
        }
        if !v[i][j-1] {
            pq.push((b[i][j-1], i, j-1)); v[i][j-1] = true;
        }
        if !v[i][j+1] {
            pq.push((b[i][j+1], i, j+1)); v[i][j+1] = true;
        }
    }

    Ok(())
}