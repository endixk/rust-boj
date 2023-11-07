// BOJ 30415 [Cat League]
// Supported by GitHub Copilot

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
    let mut p = vec![true; n + 1];
    p[0] = false;
    p[1] = false;
    let mut i = 2;
    while i * i <= n {
        if p[i] {
            let mut j = i * i;
            while j <= n {
                p[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let mut v = Vec::new();
    for i in 0..=n {
        if p[i] {
            v.push(i);
        }
    }
    v
}

fn go(v: &[usize], k: &mut Vec<usize>, x: usize, m: usize, n: usize, ans: &mut usize, ansk: &mut Vec<usize>) {
    if x == v.len() {
        for i in 0..m {
            if k[i] == 1 { return; }
        }
        let mut a = 0;
        for i in 0..m {
            a += n / k[i];
        }
        if a < *ans {
            *ans = a;
            *ansk = k.clone();
        }
        return;
    }

    for j in 0..m {
        k[j] *= v[x];
        go(v, k, x+1, m, n, ans, ansk);
        k[j] /= v[x];
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let v = sieve(n/2);
    let mut v = v.iter().filter(|&x| n % x == 0).map(|&x| x).collect::<Vec<_>>();
    for i in v.iter_mut() {
        let mut p = 1;
        while n % i.pow(p+1) == 0 { p += 1; }
        *i = i.pow(p);
    }
    if v.len() < m {
        writeln!(so, "-1")?;
        return Ok(());
    }

    let mut k = vec![1; m];
    let mut ans = usize::MAX;
    let mut ansk = vec![0; m];
    go(&v, &mut k, 0, m, n, &mut ans, &mut ansk);

    for x in ansk {
        write!(so, "{} ", n / x)?;
    }
    writeln!(so)?;

    Ok(())
}
