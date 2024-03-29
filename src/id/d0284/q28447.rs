// BOJ 28447 [Mala Soup]
pub fn main() { read();
    let (n, k) = (next::<usize>(), next::<usize>());
    let mut c = vec![vec![0; n]; n];
    for i in 0..n { for j in 0..n { c[i][j] = next(); } }

    let mut ans = -0x3f3f3f3f;
    for bit in 0..1<<n {
        let b = (0..n).filter(|&i| bit&(1<<i)!=0).collect::<Vec<_>>();
        if b.len() != k { continue; }

        let mut val = 0;
        for i in 0..k-1 { for j in i+1..k {
            val += c[b[i]][b[j]];
        }}
        ans = ans.max(val);
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