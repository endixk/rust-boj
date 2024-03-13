// BOJ 31418 [Sponge]
// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = u32; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
const MOD: u64 = 998_244_353;
pub fn main() {
    let mut p = input(16000032);
    let (w, h, k, t) = (ptr(&mut p), ptr(&mut p), ptr(&mut p), ptr(&mut p));
    let mut ans = 1;
    for _ in 0..k {
        let (i, j) = (ptr(&mut p) - 1, ptr(&mut p) - 1);
        let l = i.checked_sub(t).unwrap_or(0);
        let r = (i+t).min(w-1);
        let u = j.checked_sub(t).unwrap_or(0);
        let d = (j+t).min(h-1);
        ans = ans * (r-l+1) as u64 % MOD * (d-u+1) as u64 % MOD;
    }
    println!("{}", ans);
}