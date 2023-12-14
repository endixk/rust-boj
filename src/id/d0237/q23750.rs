// BOJ 23750 [Leader-based Team Distribution]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut a = (0..n).map(|_| (next::<usize>(), next::<usize>())).collect::<Vec<_>>();
    a.sort_unstable_by(|x, y| y.0.cmp(&x.0).then(y.1.cmp(&x.1)));

    let mut t = (0..m).map(|_| next::<usize>()).collect::<Vec<_>>();
    t.sort_unstable_by(|x, y| y.cmp(&x));

    let mut ans = a[0].1;
    let mut j = 1;
    let mut pq = std::collections::BinaryHeap::new();
    for i in 0..m-1 {
        for _ in 0..t[i] {
            pq.push(a[j].1);
            j += 1;
        }
        ans += pq.pop().unwrap();
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