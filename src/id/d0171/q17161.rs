// BOJ 17161 [Edit Distance (Hard)]
fn score(s: &[u8], t: &[u8]) -> Vec<i32> {
    let mut f = (0..=t.len()).map(|i| i as i32).collect::<Vec<_>>();
    for i in 0..s.len() {
        let mut h = vec![0; t.len()+1];
        h[0] = i as i32 + 1;
        for j in 0..t.len() {
            if s[i] == t[j] { h[j+1] = f[j]; }
            else { h[j+1] = h[j].min(f[j]).min(f[j+1]) + 1; }
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
        if f[j+1-tx] + g[j+1-tx] < f[k] + g[k] { k = j+1-tx; }
    }

    let (a, b) = hirschberg(s, sx, sm, t, tx, tx+k-1);
    let (c, d) = hirschberg(s, sm+1, sy, t, tx+k, ty);
    let z = a.into_iter().chain(c.into_iter()).collect::<Vec<_>>();
    let w = b.into_iter().chain(d.into_iter()).collect::<Vec<_>>();
    (z, w)
}
pub fn main() { read();
    let mut s = vec![0];
    s.extend(next::<String>().into_bytes());
    let mut t = vec![0];
    t.extend(next::<String>().into_bytes());
    let res = hirschberg(&s, 1, s.len()-1, &t, 1, t.len()-1);

    for (x, y) in res.0.into_iter().zip(res.1.into_iter()) {
        if x == b'-' { println!("a {}", y as char); }
        else if y == b'-' { println!("d {}", x as char); }
        else if x == y { println!("c {}", x as char); }
        else { println!("m {}", y as char); }
    }
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