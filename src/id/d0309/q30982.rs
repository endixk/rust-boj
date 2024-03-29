fn knap(a: &[usize], x: usize) -> bool {
    let mut dp = vec![false; x+1];
    dp[0] = true;
    for i in 0..a.len() {
        let mut tp = dp.clone();
        for j in 0..=x {
            if dp[j] && j+a[i] <= x { tp[j+a[i]] = true; }
        }
        dp = tp;
        if dp[x] { return true; }
    }
    false
}
pub fn main() { read();
    let (n, m, t) = (next::<usize>(), next::<usize>(), next::<usize>());
    let q = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    let p = next::<usize>()-1;
    if m < q[p] { println!("NO"); return; }
    if (p*2+(n-1-p)).min((n-1-p)*2+p) < t {
        if knap(&q, m) { println!("YES"); return; }
        else { println!("NO"); return; }
    }
    for l in 0..=p { for r in p..n {
        if ((p-l)*2+(r-p)).min((r-p)*2+(p-l)) != t { continue; }
        let mut a = vec![];
        for i in l..=r {
            if i == p { continue; }
            a.push(q[i]);
        }
        if knap(&a, m-q[p]) { println!("YES"); return; }
    }}
    println!("NO");
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