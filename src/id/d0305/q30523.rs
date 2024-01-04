// BOJ 30523 [OR & XOR]
pub fn main() { read();
    let (n, mut p) = (next::<usize>(), next::<usize>());
    let (mut x, mut y) = (0, 0);
    let mut a = vec![0; 1<<17];
    for _ in 0..n {
        let k = next::<usize>();
        x += k; a[k] += 1;
    }
    for i in 0..17 { for j in (0..1<<17).rev() {
        if j & 1 << i == 0 { a[j] += a[j | 1 << i]; }
    }}
    let mut b = vec![0; 1<<17];
    for _ in 0..n {
        let k = next::<usize>();
        y += k; b[k] += 1;
    }
    for i in 0..17 { for j in (0..1<<17).rev() {
        if j & 1 << i == 0 { b[j] += b[j | 1 << i]; }
    }}

    let mut d = (0..1<<17).map(|i| a[i] * b[i]).collect::<Vec<_>>();
    for i in 0..17 { for j in (0..1<<17).rev() {
        if j & 1 << i == 0 { d[j] -= d[j | 1 << i]; }
    }}

    let mut ans = (x + y) * n;
    for i in (0..1<<17).rev() {
        if p == 0 { ans -= d[i] * i * 2; }
        else if p < d[i] { ans -= p * i; ans -= (d[i] - p) * i * 2; p = 0; }
        else { ans -= d[i] * i; p -= d[i]; }
    }
    println!("{}", ans);
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