// BOJ 30804 [Fruit Tanghulu]
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
    let a = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();

    let mut ans = 0;
    let mut ex = vec![0; 10];
    let (mut c, mut i, mut j) = (1, 0, 0);
    ex[a[0]] = 1;
    loop {
        while c <= 2 {
            j += 1;
            if j == n { break; }
            c += if ex[a[j]] == 0 { 1 } else { 0 };
            ex[a[j]] += 1;
        }
        ans = ans.max(j - i);
        if j == n { break; }
        while c > 2 {
            c -= if ex[a[i]] == 1 { 1 } else { 0 };
            ex[a[i]] -= 1;
            i += 1;
        }
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
