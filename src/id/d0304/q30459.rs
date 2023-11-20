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

    let (n, m, r) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<i64>(&mut it)*2);
    let a = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();

    let mut d = vec![];
    for i in 0..n-1 { for j in i+1..n {
        if a[i] > a[j] { d.push(a[i] - a[j]); }
        else { d.push(a[j] - a[i]); }
    }}
    d.sort_unstable();
    d.dedup();

    let mut ans = 0;
    for _ in 0..m {
        let h = next::<i64>(&mut it);
        let i = d.partition_point(|&x| x * h <= r);
        if i == 0 { continue; }
        ans = ans.max(h * d[i-1]);
    }
    if ans == 0 { writeln!(so, "-1")?; return Ok(());}
    writeln!(so, "{:.1}", ans as f64 / 2.0)?;

    Ok(())
}