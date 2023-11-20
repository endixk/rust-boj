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
    if n % 2 == 0 {
        writeln!(so, "{}", 2 * (n/2 - 1) * (n/2 - 1))?;
        for i in 2..n/2+2 {
            writeln!(so, "{} {}", i-1, i)?;
        }
        for i in n/2+2..=n {
            writeln!(so, "{} {}", 1, i)?;
        }
    } else {
        writeln!(so, "{}", 2 * (n/2 - 1) * (n/2))?;
        for i in 2..n/2+2 {
            writeln!(so, "{} {}", i-1, i)?;
        }
        for i in n/2+2..=n {
            writeln!(so, "{} {}", 1, i)?;
        }
    }

    Ok(())
}