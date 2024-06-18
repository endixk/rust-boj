// BOJ 31673 [Exceptional Student Council]
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
    let mut p = input(1100099);
    let (n, m) = (ptr(&mut p) as usize, ptr(&mut p) as usize);
    let mut a = vec![0; n];
    let mut sum = 0;
    for i in 0..n { a[i] = ptr(&mut p); sum += a[i] as i64; }
    a.sort_unstable_by(|a, b| b.cmp(a));

    let (mut c, mut x) = (1, 0);
    while x * 2 < sum { x += a[c - 1] as i64; c += 1; }
    println!("{}", m / c);
}