// BOJ 31519 [A Cappella Recording]
pub fn main() {
    let p = &mut input(1100111);
    let (n, d) = (u32(p), u32(p));
    let mut p = (0..n).map(|_| u32(p)).collect::<Vec<_>>();
    p.sort_unstable(); p.dedup();
    let (mut ans, mut c) = (1, p[0]);
    for x in p {
        if x - c > d { ans += 1; c = x; }
    }
    println!("{}", ans);
}
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn u32(p: &mut *const u8) -> u32 { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as u32 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}