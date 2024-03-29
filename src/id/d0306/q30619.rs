// BOJ 30619 [Home Ownership]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let mut v = vec![0; n+1];
    for i in 1..=n {
        v[next::<usize>()] = i;
    }

    for _ in 0..next() {
        let (l, r) = (next::<usize>(), next::<usize>());
        let mut w = vec![0; r-l+1];
        for i in l..=r { w[i-l] = v[i];}
        w.sort();

        let mut a = v.clone();
        for i in l..=r {
            a[i] = w[i-l];
        }

        let mut ans = vec![0; n+1];
        for i in 1..=n {
            ans[a[i]] = i;
        }
        ans.iter().skip(1).for_each(|x| print!("{} ", x));
        println!();
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