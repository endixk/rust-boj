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
    let mut a = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    a.sort_unstable();

    let mut ans = 0;
    let mut r = vec![];
    let mut p = 0;
    for x in a {
        if x == p { r.push(x); }
        else { p = x; ans += 1; }
    }
    p = 0;
    for x in r {
        if x > p { p = x; ans += 1; }
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}