// BOJ 31216 [Super Prime]
fn sieve_arr(n: usize) -> Vec<bool> {
    let mut arr = vec![true; n + 1];
    arr[0] = false;
    arr[1] = false;
    for i in 2..=n {
        if arr[i] {
            for j in (i * i..=n).step_by(i) {
                arr[j] = false;
            }
        }
    }
    arr
}
fn sieve_list(n: usize) -> Vec<usize> {
    let mut arr = vec![true; n + 1];
    arr[0] = false;
    arr[1] = false;
    let mut list = Vec::new();
    for i in 2..=n {
        if arr[i] {
            list.push(i);
            for j in (i * i..=n).step_by(i) {
                arr[j] = false;
            }
        }
    }
    list
}
pub fn main() { read();
    let sa = sieve_arr(28_000);
    let sl = sieve_list(320_000);
    let sp = sl.iter().enumerate().filter(|&(i, _)| sa[i+1]).map(|(_, &x)| x).collect::<Vec<_>>();
    for _ in 0..next() {
        println!("{}", sp[next::<usize>() - 1]);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}