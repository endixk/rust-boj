// BOJ 31422 [AND, OR, XOR 2]
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
    let mut dmp = input(2200022);
    let n = ptr(&mut dmp) as u64;
    let mut r = [1; 30];
    for i in 1..30 { r[i] = r[i-1] * 2 % MOD; }
    let a = (0..n).map(|_| ptr(&mut dmp)).collect::<Vec<_>>();

    let (mut and, mut or, mut xor) = (0, 0, 0);
    for b in 0..30 {
        let r = r[b];
        let (mut p, mut z, mut e, mut c) = (0, 0, 0, 0);
        for (i, &x) in a.iter().enumerate() {
            let i = i as u64;
            let x = x as u64;
            if x & 1 << b != 0 {
                and += r * (i + 1 - p);
                or += r * (i + 1 - z) * (n - i);
                c += 1;
                z = i + 1;
            } else {
                p = i + 1;
            }
            if c & 1 == 0 {
                xor += r * e;
            } else {
                xor += r * (i + 1 - e);
                e += 1;
            }
        }
        (and, or, xor) = (and % MOD, or % MOD, xor % MOD);
    }
    println!("{} {} {}", and, or, xor);
}