// BOJ 14465 [Why Did the Cow Cross the Road II (Silver)]
pub fn main() { read();
    let (n, k, b) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut a = vec![false; n];
    (0..b).for_each(|_| a[next::<usize>() - 1] = true);

    let mut ans = (0..k).filter(|&i| a[i]).count();
    let mut c = ans;
    for i in k..n {
        if a[i] { c += 1; }
        if a[i - k] { c -= 1; }
        ans = ans.min(c);
    }
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