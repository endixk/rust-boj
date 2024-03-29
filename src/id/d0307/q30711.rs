// BOJ 30711 [Olympiad Qualification]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, p, c) = (next::<usize>(), next::<usize>(), next::<i64>());
    let mut a = (0..n).map(|i| (next::<i64>(), i)).collect::<Vec<_>>();
    a.sort_unstable();

    let mut ans = vec![""; n];
    for &(x, i) in &a {
        if n - a.partition_point(|&(k, _)| k <= x + c) >= p {
            ans[i] = "Fail"; continue;
        }
        if n - a.partition_point(|&(k, _)| k <= x - c) <= p {
            ans[i] = "Pass"; continue;
        }
        ans[i] = "Short";
    }

    println!("{}", ans.join("\n"));
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