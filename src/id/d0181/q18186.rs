// BOJ 18186 [Ramyeon Buying (Large)]
// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = i32; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
pub fn main() {
    let mut p = input(8000088);
    let (n, b, c) = (ptr(&mut p) as usize, ptr(&mut p) as i64, ptr(&mut p) as i64);
    let mut ans = 0;
    if b <= c {
        for _ in 0..n { ans += ptr(&mut p) as i64; }
        println!("{}", ans * b);
    } else {
        let (x, y, z) = (b, b+c, b+2*c);
        let (mut a, mut b, mut c) = (ptr(&mut p), ptr(&mut p), ptr(&mut p));
        for i in 0..n {
            let f = b - a;
            if f > 0 { c -= f; }
            let k = a.min(b).min(c);
            if k > 0 { ans += k as i64 * z; (a, b, c) = (a-k, b-k, c-k); }
            let k = a.min(b);
            if k > 0 { ans += k as i64 * y; (a, b) = (a-k, b-k); }
            ans += a as i64 * x;
            if f > 0 { c += f; }
            (a, b, c) = (b, c, if i < n-3 { ptr(&mut p) } else { 0 });
        }
        println!("{}", ans);
    }
}