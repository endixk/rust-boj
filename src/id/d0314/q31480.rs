// BOJ 31480 [Pigtail Virus]
// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = usize; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
pub fn main() {
    let mut p = input(3000033);
    let (n, m) = (ptr(&mut p), ptr(&mut p));
    let mut q = vec![vec![]; 35];
    for _ in 0..m {
        let (j, x, i) = (ptr(&mut p), ptr(&mut p), ptr(&mut p));
        q[i].push((j, x));
    }

    let mut v = vec![0; n+1];
    let mut p = vec![0; (n+1)<<2];
    let mut t = vec![0; (n+1)<<2];
    for k in (0..35).rev() {
        // propagate
        let (p, np) = if k & 1 == 0 { (&mut p, &mut t) } else { (&mut t, &mut p) };
        np.fill(0);
        for i in 0..(n+1)<<2 {
            if p[i] == 0 { continue; }
            let (x, i, d) = (p[i], i>>2, i&3);
            match d {
                0 if i != 1 => {
                    v[i>>1] += x; np[i>>1<<2] += x; np[i>>1<<2|if i & 1 == 0 {2} else {1}] += x;
                },
                1 if i & n+1>>1 == 0 => {
                    v[i<<1] += x; np[i<<3|1] += x; np[i<<3|2] += x;
                },
                2 if i & n+1>>1 == 0 => {
                    v[i<<1|1] += x; np[i<<3|5] += x; np[i<<3|6] += x;
                },
                _ => (),
            }
        }

        // process queries
        for &(j, x) in &q[k] {
            v[j] += x;
            np[j<<2] += x; np[j<<2|1] += x; np[j<<2|2] += x;
        }
    }

    for i in 1..=n { print!("{} ", v[i]); }
    println!();
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}