// BOJ 18115 [Dealing Cards]
pub fn main() { read();
    let n = next::<usize>();
    let mut q = std::collections::VecDeque::with_capacity(n);
    let a = (0..n).map(|_| next::<u8>()).collect::<Vec<_>>();
    for (i, x) in a.into_iter().rev().enumerate() {
        match x {
            1 => q.push_front(i + 1),
            2 => {
                q.push_front(i + 1);
                q.swap(0, 1);
            },
            _ => q.push_back(i + 1),
        }
    }
    q.iter().for_each(|x| print!("{} ", x));
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