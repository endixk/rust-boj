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

    let (mut a, mut b) = (next::<i32>(&mut it), next::<i32>(&mut it));
    let mut k = 0;
    while a > b + 1 {
        a -= 2; b -= 1; k += 1;
    }
    if a == b + 1 && b > 0 {
        writeln!(so, "YES")?;
        writeln!(so, "{}", k+1)?;
        for _ in 0..k { writeln!(so, "aba")?; }
        for _ in 0..b { write!(so, "ab")?; }
        writeln!(so, "a")?;
    } else {
        writeln!(so, "NO")?;
    }

    Ok(())
}