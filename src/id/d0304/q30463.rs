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

#[inline] fn ones(n: u64) -> u64 {
    let mut c = 0;
    let mut n = n;
    while n > 0 {
        c += n & 1;
        n >>= 1;
    }
    c
}
fn conv(s: String) -> usize {
    let mut ex = vec![false; 10];
    for c in s.chars() {
        ex[c as usize - '0' as usize] = true;
    }
    let mut n = 0;
    for i in 0..10 {
        if ex[i] { n |= 1 << i; }
    }
    n
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<u64>(&mut it));
    let mut p = vec![0; 1<<10];
    for _ in 0..n {
        let s = next::<String>(&mut it);
        p[conv(s)] += 1;
    }

    let mut ans = 0i64;
    for i in 0..1<<10 { for j in i..1<<10 {
        if ones(i | j) == k {
            if i != j {
                ans += p[i as usize] * p[j as usize];
            } else {
                ans += p[i as usize] * (p[i as usize] - 1) / 2;
            }
        }
    }}
    writeln!(so, "{}", ans)?;

    Ok(())
}