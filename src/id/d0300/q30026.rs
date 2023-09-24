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

fn cycle(a: &[usize], v: &mut[usize], i: usize, c: usize) {
    let mut j = i;
    while v[j] == 0 {
        v[j] = c;
        j = a[j];
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    for i in 0..n {
        let x = next::<usize>(&mut it) - 1;
        a[i] = x;
        b[x] = i;
    }

    let mut c = 0;
    let mut v = vec![0; n];
    for i in 0..n {
        if v[i] == 0 {
            c += 1;
            cycle(&a, &mut v, i, c);
        }
    }
    writeln!(so, "{} {}", c-1, c-1)?;

    let mut m = vec![false; c];
    m[v[0]-1] = true;
    for i in 0..n-1 {
        if !m[v[i+1]-1] {
            writeln!(so, "{} {}", b[i]+1, b[i+1]+1)?;
            m[v[i+1]-1] = true;
            b.swap(i, i+1)
        }
    }

    Ok(())
}
