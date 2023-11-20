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

    let (n, _) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut z = vec![];
    for _ in 0..n {
        let s = next::<String>(&mut it);
        let mut v = s.chars().map(|c| c == '1').collect::<Vec<_>>();
        v.dedup();
        z.push(v);
    }

    let mut m = 0;
    for v in &z {
        m = m.max(v.iter().filter(|&&b| b).count());
    }
    let mut c = 0;
    for v in z {
        if v.iter().filter(|&&b| b).count() == m {
            c += 1;
        }
    }
    writeln!(so, "{} {}", m, c)?;

    Ok(())
}