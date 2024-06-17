// BOJ 27537 [Advertisement 2]
pub fn main() { read();
    let n = next::<usize>();
    let mut v = (0..n).map(|_| (next::<i32>(), next::<i32>())).collect::<Vec<_>>();
    v.sort_unstable();

    let mut st = vec![];
    for (x, e) in v {
        if let Some(&(y, f)) = st.last() {
            if f + y >= e + x { continue; }
        } else {
            st.push((x, e));
            continue;
        }
        while let Some((y, f)) = st.pop() {
            if e + y < f + x { st.push((y, f)); break; }
        }
        st.push((x, e));
    }

    println!("{}", st.len());
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