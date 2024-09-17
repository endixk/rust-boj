// BOJ 16286 [Panokseon]
fn go(p: &[u64], x: u64, w: u64, n: usize) -> bool {
    let mut dp = vec![false; n+1];
    let (mut d, mut l, mut r) = (0, 0, 0);
    dp[0] = true;
    for i in 1..=n {
        while p[i] - p[l] > w {
            if dp[l] { d -= 1; }
            l += 1;
        }
        while p[i] - p[r] >= x {
            if dp[r] { d += 1; }
            r += 1;
        }
        if d > 0 { dp[i] = true; }
    }
    dp[n]
}
pub fn main() { read();
    let (w, n) = (next::<u64>(), next::<usize>());
    let mut p = vec![0; n+1];
    for i in 1..=n { p[i] = p[i-1] + next::<u64>(); }

    let (mut l, mut r) = (1, w+1);
    while l < r {
        let m = (l + r) >> 1;
        if go(&p, m, w, n) { l = m + 1; }
        else { r = m; }
    }
    println!("{}", (w + 1 - l).pow(2));
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}