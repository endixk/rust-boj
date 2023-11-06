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
    let v = (0..n).map(|_| (next::<usize>(&mut it), next::<usize>(&mut it))).collect::<Vec<_>>();
    let mut h = vec![0; n];
    let mut x = 0;
    for i in 0..n {
        if v[i].0 == 1 { x = v[i].1; }
        else if x > v[i].1 {
            h[i] = x - v[i].1;
            x -= v[i].1;
        }
    }

    let mut ans = 0;
    let mut x = 0x3f3f3f3f;
    for i in (0..n).rev() {
        if v[i].0 == 1 { ans += v[i].1.min(x); }
        else { x = x.min(h[i]); }
    }

    writeln!(so, "{}", ans)?;

    Ok(())
}