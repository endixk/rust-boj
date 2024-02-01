// BOJ 31265 [Military Exercise]
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let d = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();

    let mut dp = vec![false; m+1];
    dp[0] = true;
    for x in d {
        let mut tp = vec![false; m+1];
        for _ in 0..x {
            let k = next::<usize>();
            if k > m { continue; }
            for i in (0..=m-k).rev() {
                if dp[i] || tp[i] { tp[i+k] = true; }
            }
        }
        dp = tp;
    }

    if let Some(i) = dp.iter().rposition(|&x| x) { println!("{}", i); }
    else { println!("-1"); }
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