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

    let (n, k, c, r) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let b = (0..k).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let s = (0..k).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let p = (0..k).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();

    let mut ans = 0;
    let mut h = 0;
    let mut combo = 0;
    let mut skill = vec![0; k];
    for _ in 0..n {
        let i = next::<usize>(&mut it);
        if i == 0 {
            h = if h < r { 0 } else { h - r };
            combo = 0;
            continue;
        }

        h += p[i-1];
        if h > 100 {
            writeln!(so, "-1")?;
            return Ok(());
        }
        ans += (b[i-1] * (100 + combo * c) * (100 + skill[i-1] * s[i-1])) / 10000;
        combo += 1;
        skill[i-1] += 1;
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}