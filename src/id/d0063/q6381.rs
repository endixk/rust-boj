// BOJ 6381 [Entropy]
use std::collections::BinaryHeap;
pub fn main() { read();
    loop {
        let s = next::<String>();
        if s == "END" { break; }

        let mut cnt = vec![0; 27];
        for c in s.chars() {
            match c {
                '_' => cnt[26] += 1,
                _ => cnt[c as usize - 'A' as usize] += 1
            }
        }

        let mut pq = BinaryHeap::new();
        cnt.iter().enumerate().filter(|(_, &x)| x > 0).for_each(|(_, &x)| pq.push(-x));
        if pq.len() == 1 { pq.push(0); }

        let mut ans = 0;
        while let (Some(x), Some(y)) = (pq.pop(), pq.pop()) {
            ans -= x + y;
            pq.push(x + y);
        }

        println!("{} {} {:.1}", s.len() * 8, ans, 8.0 * s.len() as f64 / ans as f64);
    }
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