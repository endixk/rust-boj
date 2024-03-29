// BOJ 23741 [Cups and Balls]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let (x, y) = (next::<usize>(), next::<usize>());
    let mut e = vec![];
    for _ in 0..m {
        e.push((next::<usize>(), next::<usize>()));
    }

    let mut dp = vec![false; n+1];
    dp[x] = true;
    for _ in 0..y {
        let mut tp = vec![false; n+1];
        for &(u, v) in &e {
            if dp[u] { tp[v] = true; }
            if dp[v] { tp[u] = true; }
        }
        dp = tp;
    }

    let mut flag = true;
    dp.iter().enumerate().filter(|&(_, &b)| b).for_each(|(i, _)| {
        print!("{} ", i); flag = false;
    });
    println!("{}", if flag { "-1" } else { "" });
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