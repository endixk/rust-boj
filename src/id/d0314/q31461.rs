// BOJ 31461 [Chocolate Pilgrimage]
// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn ptr(p: &mut *const u8) -> i32 { unsafe {
    let mut n = 0;
    let mut f = true;
    if **p == b'-' { f = false; *p = p.offset(1) }
    while **p & 16 != 0 { n = n * 10 + (**p as i32 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    if f { n } else { -n }
}}
pub fn main() {
    let mut buf = input(8888888);
    let mut p = [0; 200000];
    let mut q = [0; 200000];
    for _ in 0..ptr(&mut buf) {
        let n = ptr(&mut buf) as usize;
        (0..n).for_each(|i| p[i] = ptr(&mut buf));
        (0..n).for_each(|i| q[i] = ptr(&mut buf));
        let (a, b) = (ptr(&mut buf) as usize - 1, ptr(&mut buf) == 1);
        let (c, d) = (ptr(&mut buf) as usize - 1, ptr(&mut buf) == 1);
        let (a, b, c, d) = if a > c { (c, d, a, b) } else { (a, b, c, d) };

        let mut ans = 0;
        if a == c {
            let mut t = 0;
            for i in (0..a).rev() {
                ans = ans.max(t + p[i]).max(t + q[i]).max(t + p[i] + q[i]);
                t += p[i] + q[i];
            }
            t = 0;
            for i in a+1..n {
                ans = ans.max(t + p[i]).max(t + q[i]).max(t + p[i] + q[i]);
                t += p[i] + q[i];
            }
        } else {
            let (mut l, mut r, mut m) = (0, 0, 0);

            let mut t = if b { q[a] } else { p[a] };
            l = l.max(t);
            for i in (0..a).rev() {
                l = l.max(t + p[i]).max(t + q[i]).max(t + p[i] + q[i]);
                t += p[i] + q[i];
            }

            t = if d { q[c] } else { p[c] };
            r = r.max(t);
            for i in c+1..n {
                r = r.max(t + p[i]).max(t + q[i]).max(t + p[i] + q[i]);
                t += p[i] + q[i];
            }

            for i in a+1..c {
                m += p[i].max(q[i]).max(p[i] + q[i]);
            }

            ans = l + m + r;
        }

        println!("{}", ans + if b { p[a] } else { q[a] } + if d { p[c] } else { q[c] });
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}