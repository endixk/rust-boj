// BOJ 23742 [Player-based Team Distribution]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let (mut k, mut c, mut p) = (0, 0, vec![]);
    for _ in 0..n {
        let x = next::<i64>();
        if x >= 0 { k += x; c += 1; }
        else { p.push(x); }
    }
    p.sort_unstable_by(|a, b| b.cmp(a));

    let mut ans = k * c;
    let mut i = 0;
    while i < p.len() {
        if k + p[i] * (c + 1) >= p[i] {
            ans += k + p[i] * (c + 1);
            k += p[i];
            c += 1; i += 1;
        } else {
            break;
        }
    }

    ans += p[i..].iter().sum::<i64>();
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