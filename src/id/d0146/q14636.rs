// BOJ 14636 [Money for Nothing]
fn dnco(ans: &mut i64, p: &[(i64, i64)], q: &[(i64, i64)], s: usize, e: usize, l: usize, r: usize) {
    if s > e { return; }
    let m = s + e >> 1;

    let (mut x, mut j) = (i64::MIN, l);
    for i in l..=r {
        let dx = q[i-1].0 - p[m-1].0;
        let dy = q[i-1].1 - p[m-1].1;
        let v = if dx < 0 && dy < 0 { -1 } else { 1 } * dx * dy;
        if x < v { x = v; j = i; }
    }

    if *ans < x { *ans = x; }
    dnco(ans, p, q, s, m - 1, l, j);
    dnco(ans, p, q, m + 1, e, j, r);
}
pub fn main() { read();
    let (m, n) = (next::<usize>(), next::<usize>());

    let mut p = (0..m).map(|_| (next::<i64>(), next::<i64>())).collect::<Vec<_>>();
    p.sort_unstable();
    let mut tp = vec![];
    let mut ymin = 0x3f3f3f3f;
    for (x, y) in p {
        if y >= ymin { continue; }
        ymin = y; tp.push((x, y));
    }
    let p = tp;

    let mut q = (0..n).map(|_| (next::<i64>(), next::<i64>())).collect::<Vec<_>>();
    q.sort_unstable_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
    let mut tq = vec![];
    let mut ymax = 0;
    for (x, y) in q {
        if y <= ymax { continue; }
        ymax = y; tq.push((x, y));
    }
    tq.reverse();
    let q = tq;

    let mut ans = 0;
    dnco(&mut ans, &p, &q, 1, p.len(), 1, q.len());
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