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
    let a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let mut d = vec![a[0], -0x3f3f3f3f, -0x3f3f3f, a[0]*2];
    for i in 1..n {
        let mut t = vec![0; 4];
        // push now
        t[3] = d[0].max(d[1]) + a[i]*2;
        // do not push
        t[2] = d[3] + a[i]*2;
        t[1] = d[2] + a[i]*2;
        t[0] = d[0].max(d[1]) + a[i];
        d = t;
    }
    writeln!(so, "{}", d.iter().max().unwrap())?;

    Ok(())
}