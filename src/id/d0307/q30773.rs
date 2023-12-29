// BOJ 30773 [Mikhail Strikes Back]
fn score(s: &[u8], t: &[u8]) -> Vec<i32> {
    let mut f = vec![0; t.len()+1];
    for i in 0..s.len() {
        let mut h = vec![0; t.len()+1];
        for j in 0..t.len() {
            h[j+1] = f[j+1].max(h[j]);
            if s[i] == t[j] { h[j+1] = h[j+1].max(f[j] + 1); }
        }
        f = h;
    }
    f
}
fn hirschberg(s: &[u8], sx: usize, sy: usize,
              t: &[u8], tx: usize, ty: usize) -> (Vec<u8>, Vec<u8>) {
    if sx > sy { return (vec![b'-'; ty-tx+1], t[tx..=ty].to_vec()); }
    if tx > ty { return (s[sx..=sy].to_vec(), vec![b'-'; sy-sx+1]); }
    if sx == sy {
        let mut z = vec![b'-'; ty-tx+1];
        if let Some(p) = t[tx..=ty].iter().position(|&c| c == s[sx]) {
            z[p] = s[sx];
        } else { z[0] = s[sx]; }
        return (z, t[tx..=ty].to_vec());
    }
    if tx == ty {
        let mut z = vec![b'-'; sy-sx+1];
        if let Some(p) = s[sx..=sy].iter().position(|&c| c == t[tx]) {
            z[p] = t[tx];
        } else { z[0] = t[tx]; }
        return (s[sx..=sy].to_vec(), z);
    }

    let sm = (sx + sy) >> 1;

    let f = score(&s[sx..=sm], &t[tx..=ty]);
    let mut g = score(&s[sm+1..=sy].iter().rev().cloned().collect::<Vec<_>>(),
                      &t[tx..=ty].iter().rev().cloned().collect::<Vec<_>>());
    g.reverse();

    let mut k = 0;
    for j in tx..=ty {
        if f[j+1-tx] + g[j+1-tx] > f[k] + g[k] { k = j+1-tx; }
    }

    let (a, b) = hirschberg(s, sx, sm, t, tx, tx+k-1);
    let (c, d) = hirschberg(s, sm+1, sy, t, tx+k, ty);
    let z = a.into_iter().chain(c.into_iter()).collect::<Vec<_>>();
    let w = b.into_iter().chain(d.into_iter()).collect::<Vec<_>>();
    (z, w)
}
pub fn main() { read();
    let s = next::<String>().into_bytes();
    let mut u = vec![0]; u.extend(s.iter());
    let mut v = vec![1]; v.extend(s.iter().rev());
    let res = hirschberg(&u, 1, u.len()-1, &v, 1, v.len()-1);

    // debug
    // println!("{}\n{}", String::from_utf8_lossy(&res.0), String::from_utf8_lossy(&res.1));

    let mut lcs = vec![];
    for (x, y) in res.0.into_iter().zip(res.1.into_iter()) {
        if x == y && x != b'-' { lcs.push(x); }
    }
    println!("{}", s.len() - lcs.len());

    let mut split = vec![];
    let mut buf = vec![];
    let mut i = 0;
    for x in s {
        if i < lcs.len() && x == lcs[i] {
            split.push(buf); buf = vec![];
            split.push(vec![x]);
            i += 1;
        } else { buf.push(x); }
    }
    split.push(buf);

    let k = split.len();
    let mut res = split.clone();
    for i in 0..k {
        if i % 2 == 0 {
            res[k-i-1].extend(split[i].iter().rev());
        }
    }
    println!("{}", res.iter().flatten().map(|&c| c as char).collect::<String>());

}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}