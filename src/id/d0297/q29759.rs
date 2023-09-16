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

fn sieve(n: usize) -> Vec<usize> {
    let mut isp = vec![true; n+1];
    isp[0] = false; isp[1] = false;
    for i in 2..n+1 {
        if !isp[i] { continue; }
        for j in 2..n/i+1 {
            isp[i*j] = false;
        }
    }
    isp.into_iter().enumerate().filter(|&(_, b)| b).map(|(i, _)| i).collect()
}
fn factorize(n: usize, p: &[usize]) -> Vec<usize> {
    let mut ret = vec![];
    let mut n = n;
    for &i in p {
        if i * i > n { break; }
        if n % i == 0 {
            ret.push(i);
            while n % i == 0 { n /= i; }
        }
    }
    if n != 1 { ret.push(n); }
    ret
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let p = sieve(1000001);
    let mut ex = vec![false; 1000001];
    let mut b = vec![vec![0; n*n]; n*n];
    for i in 0..n*n {
        for j in 0..n*n {
            b[i][j] = next::<usize>(&mut it);
            for f in factorize(b[i][j], &p) {
                ex[f] = true;
            }
        }
    }

    let p = sieve(1000001);
    let mut k = 0;
    for i in 0..n*n {
        for j in 0..n*n {
            if b[i][j] != 0 { write!(so, "{} ", b[i][j]).unwrap(); continue; }
            else {
                while ex[p[k]] { k += 1; }
                write!(so, "{} ", p[k]).unwrap();
                k += 1;
            }
        }
        writeln!(so, "").unwrap();
    }
}