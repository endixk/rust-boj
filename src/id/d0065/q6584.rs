// BOJ 6584 [The Settlers of Catan]
fn go(u: usize, e: &Vec<(usize, usize)>, v: &mut Vec<bool>) -> usize {
    let mut ret = 0;
    for (i, &(p, q)) in e.iter().enumerate() {
        if v[i] { continue; }
        if p == u {
            v[i] = true;
            ret = ret.max(1 + go(q, e, v));
            v[i] = false;
        } else if q == u {
            v[i] = true;
            ret = ret.max(1 + go(p, e, v));
            v[i] = false;
        }
    }
    ret
}
pub fn main() { read();
    loop {
        let (n, m) = (next::<usize>(), next::<usize>());
        if n == 0 && m == 0 { break; }

        let e = (0..m).map(|_| (next::<usize>(), next::<usize>())).collect::<Vec<_>>();
        let mut v = vec![false; m];
        let mut ans = 0;
        for i in 0..n {
            ans = ans.max(go(i, &e, &mut v));
        }
        println!("{}", ans);
    }
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