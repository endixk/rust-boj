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

    let (a, b, n) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    if b / 10 == 2 || b / 10 == 4 || b / 10 == 5 || b / 10 == 6 || b / 10 == 8 {
        writeln!(so, "-1")?;
        return Ok(());
    }
    if a % 10 == 9 {
        write!(so, "{}7", a)?;
        for _ in 0..n-5 { write!(so, "1")?; }
        write!(so, "{}", b)?;
    } else {
        write!(so, "{}", a)?;
        for _ in 0..n-4 { write!(so, "1")?; }
        write!(so, "{}", b)?;
    }

    Ok(())
}