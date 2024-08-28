// BOJ 1507 [Curious Minho]
const INF: u32 = 0x3f3f3f3f;
pub fn main() { read();
    let n: usize = next();
    let mut d = [INF; 1<<10];
    let mut b = [false; 1<<10];
    for i in 0..n { for j in 0..n {
        let x = next::<u32>();
        if i > j { continue; }
        b[i<<5|j] = true; d[i<<5|j] = x;
    }}
    for i in 0..n-2 { for j in i+1..n-1 { for k in j+1..n {
        let (ij, jk, ik) = (i<<5|j, j<<5|k, i<<5|k);
        if d[ij] > d[ik] + d[jk] { println!("-1"); return; }
        if d[ij] == d[ik] + d[jk] { b[ij] = false; }
        if d[jk] > d[ij] + d[ik] { println!("-1"); return; }
        if d[jk] == d[ij] + d[ik] { b[jk] = false; }
        if d[ik] > d[ij] + d[jk] { println!("-1"); return; }
        if d[ik] == d[ij] + d[jk] { b[ik] = false; }
    }}}
    println!("{}", b.iter().enumerate().filter(|(_, &x)| x).map(|(i, _)| d[i]).sum::<u32>());
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
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