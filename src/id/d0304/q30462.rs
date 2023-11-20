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
    let mut v = std::collections::BTreeSet::new();
    (1..=n).for_each(|i| { v.insert(i); });

    let mut b = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    b.reverse();

    if b[0] != n+1 { writeln!(so, "No")?; return Ok(()); }
    let mut a = vec![0; n];
    for i in 1..n {
        if b[i] > b[i-1] { writeln!(so, "No")?; return Ok(()); }
        let k = if let Some(x) = v.range(b[i]..).next() { *x }
        else { writeln!(so, "No")?; return Ok(()); };
        v.remove(&k);
        a[n-i] = k;
    }
    a[0] = *v.iter().next().unwrap();

    writeln!(so, "Yes")?;
    for x in a {
        write!(so, "{} ", x)?;
    }
    writeln!(so)?;

    Ok(())
}