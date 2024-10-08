// BOJ 31674 [Exceptional Technology]
// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = u32; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
const MOD: u64 = 1000000007;
pub fn main() {
    let mut p = input(2200022);
    let n = ptr(&mut p) as usize;
    let mut v = (0..n).map(|_| ptr(&mut p)).collect::<Vec<_>>();
    v.sort_unstable();
    let (mut k, mut b) = (0, 1);
    for x in v {
        k = (k + x as u64 * b) % MOD;
        b = b * 2 % MOD;
    }
    println!("{}", k);
}