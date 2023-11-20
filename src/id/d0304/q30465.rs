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

fn cycle(a: &[usize], v: &mut Vec<bool>, x: usize, n: usize) -> (usize, usize) {
    let (mut c, mut t) = (0, 0);
    let mut i = x;
    while !v[i] {
        v[i] = true;
        c += 1;
        if i <= n/2 { t |= 1; }
        else { t |= 2; }
        i = a[i];
    }
    (c, t)
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut a = vec![0; n+1];
    for i in 1..=n {
        a[i] = next::<usize>(&mut it);
    }
    if n%2 == 1 && a[(n+1)/2] != (n+1)/2 {
        writeln!(so, "-1")?;
        return Ok(());
    }

    let mut v = vec![false; n+1];
    let mut w = vec![vec![]; 4];
    for i in 1..=n {
        if !v[i] {
            let (c, t) = cycle(&a, &mut v, i, n);
            if c > 1 { w[t].push(c); }
        }
    }

    let mut ans = 0;
    while !w[1].is_empty() && !w[2].is_empty() {
        ans += 1;
        let (x, y) = (w[1].pop().unwrap(), w[2].pop().unwrap());
        w[3].push(x+y);
    }
    while let Some(x) = w[1].pop() {
        ans += 1;
        w[3].push(x+1);
    }
    while let Some(x) = w[2].pop() {
        ans += 1;
        w[3].push(x+1);
    }
    while let Some(x) = w[3].pop() {
        ans += x-1;
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}