// BOJ 1461 [Library]
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let (mut x, mut y) = (vec![], vec![]);
    for _ in 0..n {
        match next::<i32>() {
            v if v < 0 => x.push(-v),
            v => y.push(v),
        }
    }
    x.push(0); y.push(0);
    x.sort_by_key(|&v| -v);
    y.sort_by_key(|&v| -v);

    let mut ans = 0;
    (0..x.len()).step_by(m).for_each(|i| ans += x[i] * 2);
    (0..y.len()).step_by(m).for_each(|i| ans += y[i] * 2);
    ans -= x[0].max(y[0]);
    println!("{}", ans);
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
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