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

    let (n, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let v = (0..k).map(|_| (next::<i32>(&mut it), next::<i32>(&mut it))).collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..k-1 {
        for j in i+1..k {
            ans = ans.max(((v[i].0 - v[j].0).abs() + (v[i].1 - v[j].1).abs() - 1) / 2);
        }
    }
    for i in 1..=n {
        let (mut x, mut y) = (200000, 200000);
        for j in 0..k {
            x = x.min((i as i32 - v[j].0).abs() + (v[j].1 - 1).abs());
            y = y.min((i as i32 - v[j].0).abs() + (v[j].1 - m as i32).abs());
        }
        ans = ans.max(x).max(y);
    }
    for i in 1..=m {
        let (mut x, mut y) = (200000, 200000);
        for j in 0..k {
            x = x.min((i as i32 - v[j].1).abs() + (v[j].0 - 1).abs());
            y = y.min((i as i32 - v[j].1).abs() + (v[j].0 - n as i32).abs());
        }
        ans = ans.max(x).max(y);
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}