// BOJ 6557 [Hall of Fountains]
pub fn main() { read();
    'x: loop {
        let n = next::<usize>();
        if n == 0 { break; }
        let mut a = vec![0; n+2];
        for i in 1..=n { a[i] = next::<usize>(); }
        let mut b = vec![0; n+2];
        for i in 1..=n { b[i] = next::<usize>(); }

        let mut dp = vec![false; n+2];
        dp[0] = true;
        for t in 1..=90*n {
            let mut tp = vec![false; n+2];
            if a[1] == 0 || (t + 2*a[1] - b[1]) / a[1] % 2 == 1 {
                tp[1] = true;
            }
            for i in 1..=n {
                if !dp[i] { continue; }
                if !tp[i-1] && (a[i-1] == 0 || (t + 2*a[i-1] - b[i-1]) / a[i-1] % 2 == 1) {
                    tp[i-1] = true;
                }
                if !tp[i] && (a[i] == 0 || (t + 2*a[i] - b[i]) / a[i] % 2 == 1) {
                    tp[i] = true;
                }
                if !tp[i+1] && (a[i+1] == 0 || (t + 2*a[i+1] - b[i+1]) / a[i+1] % 2 == 1) {
                    tp[i+1] = true;
                }
            }
            dp = tp;
            if dp[n+1] { println!("{}", t); continue 'x; }
        }
        println!("0");
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}