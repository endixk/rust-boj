// BOJ 16161 [Longest Increasing Palindromic Subsequence]
fn count(a: &[u32], i: usize, j: usize, n: usize) -> usize {
    let (mut i, mut j) = (i, j);
    loop {
        if i == 0 || j == n-1 { break; }
        if a[i-1] != a[j+1] { break; }
        if a[i-1] >= a[i] { break; }
        i -= 1; j += 1;
    }
    j - i + 1
}
pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<u32>()).collect::<Vec<_>>();
    let mut ans = 1;

    let mut i = 0;
    while i < n-1 {
        let mut r = count(&a, i, i, n);
        if a[i] == a[i+1] { r = r.max(count(&a, i, i+1, n)); }
        ans = ans.max(r);
        i += r;
    }
    println!("{}", ans);
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}