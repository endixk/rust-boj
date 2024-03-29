// BOJ 30882 [Pay2Win]
pub fn main() { read();
    const INF: usize = 1<<60;
    let (n, k, h) = (next::<usize>(), next::<usize>(), next::<usize>());
    let p = (0..k).map(|_| (
        next::<usize>() - 1, next::<usize>() - 1, next::<usize>()
    )).collect::<Vec<_>>();

    let mut dp = vec![INF; n];
    dp[n-1] = 0;
    for (i, j, x) in p.into_iter().rev() {
        let (ti, tj) = (dp[i], dp[j]);
        dp[i] += x; if tj != INF { dp[i] = dp[i].min(tj); }
        dp[j] += x; if ti != INF { dp[j] = dp[j].min(ti); }
    }

    println!("{}", dp.iter().filter(|&&x| x < INF).max().unwrap() + (h - 1) * dp[n-1]);
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