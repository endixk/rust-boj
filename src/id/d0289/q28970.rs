// BOJ 28970 [Card Trick]
// Supported by GitHub Copilot

pub fn main() { read();
    let (mut p, mut q, mut r, mut s) = (next::<i64>(), next::<i64>(), next::<i64>(), next::<i64>());
    let mut cnt = 0;
    const SQRT: i64 = 31622;
    loop {
        let (a, b) = if p > q { (p, q) } else { (q, p) };
        let (c, d) = if r > s { (r, s) } else { (s, r) };
        if b == 0 && d == 0 { break; }
        if b == 0 || d == 0 || c < b || a < d { println!("NO"); return; }
        if (a-b)/d == 0 && (c-d)/b > SQRT*2 {
            (p, q, r, s) = (a - SQRT*d, b - SQRT*d, c - SQRT*a - SQRT*b + SQRT*SQRT*d, d);
            cnt += SQRT*2;
        } else if (a-b)/d > SQRT*2 && (c-d)/b == 0 {
            (p, q, r, s) = (a - SQRT*c - SQRT*d + SQRT*SQRT*b, b, c - SQRT*b, d - SQRT*b);
            cnt += SQRT*2;
        } else {
            let k = ((a - b) / d).min((c - d) / b).max(1);
            (p, q, r, s) = (a - k * d, b, c - k * b, d);
            cnt += k;
        }

    }
    println!("YES\n{}", cnt);
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