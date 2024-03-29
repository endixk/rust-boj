// BOJ 30400 [Removing Palindromes]
// Supported by GitHub Copilot

fn manacher(s: &[u8], n: usize) -> Vec<usize> {
    let mut a = vec![0; n];
    let (mut r, mut p) = (0, 0);
    for i in 0..n {
        if i <= r {
            a[i] = a[2 * p - i].min(r - i);
        }
        while i > a[i] && i + a[i] + 1 < n && s[i - a[i] - 1] == s[i + a[i] + 1] {
            a[i] += 1;
        }
        if r < i + a[i] {
            r = i + a[i];
            p = i;
        }
    }
    a
}

pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut s = vec![0; 2*n+1];
    let t = next::<String>();
    let t = t.as_bytes();
    t.iter().enumerate().for_each(|(i, &c)| { s[2*i+1] = c; });

    let a = manacher(&s, 2*n+1);
    // println!("{:?}", a);

    let mut i = 1;
    let mut ans = 0;
    while i <= 2*n {
        if a[i] >= m {
            ans += 1;
            if i % 2 == 1 {
                i += m * 2 + if m % 2 == 0 { 1 } else { 0 };
            } else {
                i += m * 2 + m % 2;
            }
        } else {
            i += 1;
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