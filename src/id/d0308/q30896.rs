// BOJ 30896 [Team Selection]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let mut v = (0..n).map(|_| (next::<usize>(), next::<usize>())).collect::<Vec<_>>();
    v.sort_unstable();

    let sum = v.iter().map(|x| x.1).sum::<usize>();
    let mut w = vec![false; sum + 1];
    let mut ans = 0x3f3f3f3f;
    w[0] = true;
    for i in (1..n).rev() {
        let mut t = vec![false; sum + 1];
        for p in 0..=sum {
            if !w[p] { continue; }
            let s = p + v[i].1;
            let (l, r) = (v[i].0 * s, v[0].0 * (sum - s));
            ans = ans.min(if l > r { l - r } else { r - l });
            t[p] = true; t[s] = true;
        }

        t[v[i].1] = true;
        w = t;
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