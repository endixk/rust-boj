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
    let mut p = vec![0; n];
    for _ in 0..m {
        let (k, s, e) = (next::<usize>(&mut it)-1, next::<usize>(&mut it), next::<usize>(&mut it));
        if p[k] <= s {
            writeln!(so, "YES")?;
            p[k] = e;
        } else {
            writeln!(so, "NO")?;
        }
    }

    Ok(())
}