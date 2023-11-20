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

    let (n, m, q) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec![vec![0; m]; n];
    for i in 0..n { for j in 0..m {
        b[i][j] = next::<i32>(&mut it);
    }}

    let mut p = vec![vec![0; m]; n];
    p[0] = b[0].clone();
    for i in 1..n { for j in 0..m {
        p[i][j] = p[i-1][j] + b[i][j];
    }}

    let mut a = vec![vec![0; m]; n];
    for i in 0..n { a[i][0] = p[i][0]; }
    for j in 1..m { a[0][j] = p[0][j]; }
    for i in 1..n { for j in 1..m {
        a[i][j] = p[i][j] + a[i-1][j-1];
    }}

    for _ in 0..q {
        writeln!(so, "{}", a[next::<usize>(&mut it)-1][next::<usize>(&mut it)-1])?;
    }

    Ok(())
}