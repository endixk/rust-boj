// BOJ 28141 [Easy Interactive Problem]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};
fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn sieve(n: usize) -> Vec<u32> {
    let mut p = vec![true; 1000010];
    p[0] = false; p[1] = false;
    for i in 2..1000 {
        if p[i] {
            for j in (i*i..1000010).step_by(i) {
                p[j] = false;
            }
        }
    }

    let mut ret = Vec::new();
    for i in 0..n {
        let mut s = (i + 1) * 1000;
        while !p[s] { s += 1; }
        ret.push(s as u32);
    }
    ret
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let n = read(&mut si).parse::<usize>().unwrap();
    let p = sieve(n);

    let mut ans = vec![0; n+1];
    for i in 1..=n {
        if ans[i] > 0 { continue; }

        let mut p = p[i-1];
        writeln!(so, "? {} {}", i, p).ok();
        so.flush().unwrap();
        let x = read(&mut si).parse::<usize>().unwrap();
        if i == x { ans[i] = i; continue; }

        let mut cyc = Vec::new();
        loop {
            p += 1;
            writeln!(so, "? {} {}", i, p).ok();
            so.flush().unwrap();
            let y = read(&mut si).parse::<usize>().unwrap();
            cyc.push(y);
            if y == x { break; }
        }
        for j in 0..cyc.len() {
            ans[cyc[j]] = cyc[(j+1) % cyc.len()];
        }
    }

    write!(so, "!").ok();
    for i in 1..=n {
        write!(so, " {}", ans[i]).ok();
    }
    writeln!(so).ok();
    so.flush().unwrap();
}
