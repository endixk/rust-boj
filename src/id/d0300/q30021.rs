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
    if n == 1 {
        writeln!(so, "YES")?;
        writeln!(so, "1")?;
    } else if n == 2 {
        writeln!(so, "NO")?;
    } else {
        writeln!(so, "YES")?;
        write!(so, "1 3 2")?;
        for i in 4..=n { write!(so, " {}", i)?; }
        writeln!(so)?;
    }

    Ok(())
}