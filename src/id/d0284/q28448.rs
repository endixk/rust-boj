// BOJ 28448 [PS Madness]

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
    let mut p = input(1<<24);
    let (n, l) = (ptr(&mut p) as u64, ptr(&mut p) as u64);
    let mut ans = 0;
    let mut v = Vec::new();
    for _ in 0..n {
        let (k, t) = (ptr(&mut p) as u64, ptr(&mut p) as u64);
        if t <= 5 { ans += t; }
        else { v.push(k<<32|t); }
    }
    v.sort_unstable_by(|a, b| b.cmp(a));

    let mut m = 0;
    for x in v {
        let (k, t) = (x>>32, x&0xffffffff);
        if m > l - k*t { ans += m + k*t - l; m = l - k*t; }
        ans += t; m += k*(t-5);
    }
    println!("{}", ans);
}