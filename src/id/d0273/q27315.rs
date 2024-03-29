// BOJ 27315 [All In]
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut v = Vec::<u64>::with_capacity(n);
    for _ in 0..n {
        let (d, p) = (next::<u64>(), next::<u64>());
        let (t, e) = (next::<u8>() == 1, next::<u8>() == 1);
        v.push(if e { (d+1)>>1 } else { d } << 32 | if t { 0 } else if e { p>>1 } else { p });
    }
    v.sort_unstable();

    let (mut hd, mut hp) = (next::<i32>(), next::<i32>());
    let mut i = 0;
    let mut ans = 0;
    let mut pq = std::collections::BinaryHeap::<i32>::new();
    for _ in 0..m {
        while i < n && (v[i]>>32) as i32 <= hd { pq.push(-((v[i] & 0xffffffff) as i32)); i += 1; }
        if let Some(p) = pq.pop() {
            ans += if -p > hp { -p-hp } else { 0 } as u64;
        } else {
            println!("-1"); return;
        }
        hd += 1; hp += 1;
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