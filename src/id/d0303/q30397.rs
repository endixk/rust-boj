// BOJ 30397 [DSHS]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let mut a = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    let mut b = std::collections::BTreeSet::new();
    let mut c = std::collections::HashMap::new();
    for _ in 0..n {
        let x = next::<usize>();
        b.insert(x);
        *c.entry(x).or_insert(0) += 1;
    }
    a.sort_unstable();

    let mut ans = 0;
    let mut p = vec![];
    for x in a {
        if let Some(&y) = b.range(..x).next_back() {
            ans += 100;
            let k = c.get_mut(&y).unwrap();
            *k -= 1;
            if *k == 0 { b.remove(&y); }
        } else {
            p.push(x);
        }
    }
    for x in p {
        if b.contains(&x) {
            ans += 20;
            let k = c.get_mut(&x).unwrap();
            *k -= 1;
            if *k == 0 { b.remove(&x); }
        } else {
            ans -= 50;
        }
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