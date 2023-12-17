// BOJ 28959 [Sale!]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, e, w) = (next::<usize>(), next::<i64>(), next::<usize>());
    let mut a = (0..n).map(|_| next::<i64>()).collect::<Vec<_>>();
    a.sort_unstable();

    let mut p = vec![0; n+1];
    p[1] = a[0];
    for i in 1..n { p[i+1] = p[i] + a[i]; }
    // a[s..e] = p[e+1] - p[s]

    'q: for q in 1..=n {
        if w + q > n { print!("{} ", p[n] + e); continue; }
        let mut ans = 0;
        let mut i = n-w-q;
        loop {
            if p[i+w] - p[i] > e + if i >= w+q { 0 } else { p[w] } {
                ans += p[i+w+q] - p[i+w] + e;
                if i == 0 { print!("{} ", ans); continue 'q; }
                else if i < w+q { i -= 1; break; }
                else { i -= w+q; }
            } else { i += w+q-1; break; }
        }
        if i+1 >= w+q {
            print!("{} ", ans + p[i+1] - p[w] + e);
        } else {
            print!("{} ", ans + p[i+1] + e);
        }
    }
    println!();
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