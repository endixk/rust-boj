// BOJ 31675 [Exceptional Physical Attack]
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
    let mut p = input(1100011);
    let n = ptr(&mut p) as usize;

    let mut a = [0; 100005];
    let mut s = 0;
    for i in 0..n { a[i] = ptr(&mut p); s += a[i] as u64; }
    if n == 1 { println!("{}", 0); return; }

    let mut dp = [0; 100005];
    dp[n-1] = a[n-1] as u64;
    for i in (0..n-1).rev() {
        dp[i] = (a[i] as u64 + dp[i+2]).min(a[i+1] as u64 + dp[i+3]);
    }
    println!("{}", s - dp[0]);
}