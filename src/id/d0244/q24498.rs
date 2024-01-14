// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = u32; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}

pub fn main() {
    let mut p = input(10485760);
    let n = ptr(&mut p);
    if n == 1 { println!("{}", ptr(&mut p)); return; }
    if n == 2 { println!("{}", ptr(&mut p).max(ptr(&mut p))); return; }

    let (mut a, mut b, mut c) = (ptr(&mut p), ptr(&mut p), ptr(&mut p));
    let mut ans = a.max(b + a.min(c));
    for _ in 3..n {
        (a, b, c) = (b, c, ptr(&mut p));
        ans = ans.max(b + a.min(c));
    }
    println!("{}", ans.max(c));
}