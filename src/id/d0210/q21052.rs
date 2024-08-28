// BOJ 21052 [Film Critics]
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn ptr(p: &mut *const u8) -> usize { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as usize & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
pub fn main() {
    let mut buf = input(1222222);
    let (n, m, k) = (ptr(&mut buf), ptr(&mut buf), ptr(&mut buf));
    if k % m != 0 { println!("impossible"); return; }
    let x = k / m;

    let mut a = (0..n).map(|i| (ptr(&mut buf), i+1)).collect::<Vec<_>>();
    a.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let mut ans = vec![0; n];
    let (mut p, mut q) = (x, x);
    let mut tot = 0;
    for i in 0..n {
        if p > 0 && a[p-1].0 * i >= tot {
            tot += m; p -= 1; ans[i] = a[p].1;
        }
        else if q == n || a[q].0 * i >= tot { println!("impossible"); return; }
        else {
            ans[i] = a[q].1; q += 1;
        }
    }
    ans.iter().for_each(|&x| print!("{} ", x));
    println!();
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}