// BOJ 17299 [NGF]
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut a = vec![0; n];
    let mut f = vec![0; 1000001];
    for i in 0..n {
        let x = next::<usize>(&mut it);
        a[i] = x; f[x] += 1;
    }

    let mut r = vec![-1; n];
    let mut v = Vec::new();
    for i in 0..n {
        while !v.is_empty() {
            let j = *v.last().unwrap();
            if f[a[j]] < f[a[i]] { r[j] = a[i] as i32; v.pop(); }
            else { break; }
        }
        v.push(i);
    }

    r.iter().for_each(|&x| { write!(so, "{} ", x).ok(); } );
    writeln!(so)?;

    Ok(())
}
