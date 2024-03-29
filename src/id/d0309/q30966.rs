// BOJ 30966 [Common Interests]

pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    let mut dp = vec![0u32; 1<<24];
    a.iter().for_each(|&x| dp[x] += 1);

    for i in 0..24 { for j in (0..1<<24).rev() {
        if j & 1 << i != 0 { dp[j ^ 1 << i] += dp[j]; }
    }}

    dp.iter_mut().enumerate().for_each(|(i, x)| if *x > 1 { *x = i.count_ones() } else { *x = 0 });
    for i in 0..24 { for j in 0..1<<24 {
        if j & 1 << i != 0 { dp[j] = dp[j].max(dp[j ^ 1 << i]); }
    }}

    println!("{}", a.iter().map(|&x| dp[x]).sum::<u32>());
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