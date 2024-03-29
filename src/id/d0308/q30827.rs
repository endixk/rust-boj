// BOJ 30827 [Meeting Rooms]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, k) = (next::<usize>(), next::<usize>());
    let mut v = (0..n).map(|_| (next::<usize>(), next::<usize>())).collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let mut d = vec![0; k];
    let mut ans = 0;
    for (s, e) in v {
        let (mut a, mut b) = (0, k);
        for i in 0..k {
            if d[i] < s && (b == k || d[i] > a) { a = d[i]; b = i; }
        }
        if b < k { d[b] = e; ans += 1; }
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}