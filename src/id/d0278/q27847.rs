// BOJ 27847 [Cow-libi]
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn ptr(p: &mut *const u8) -> i64 { unsafe {
    let mut n = 0;
    let f = if **p == 45 { *p = p.offset(1); -1 } else { 1 };
    while **p & 16 != 0 { n = n * 10 + (**p as i64 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    f * n
}}
pub fn main() {
    let mut p = input(7777777);
    let (g, n) = (ptr(&mut p) as usize, ptr(&mut p) as usize);
    let mut v = vec![];
    for _ in 0..g {
        let (x, y, t) = (ptr(&mut p), ptr(&mut p), ptr(&mut p));
        v.push((t, x, y));
    }
    v.push((-1<<30, 0, 0)); v.push((1<<31, 0, 0));
    v.sort_unstable();

    let mut c = vec![];
    for _ in 0..n {
        let (x, y, t) = (ptr(&mut p), ptr(&mut p), ptr(&mut p));
        c.push((t, x, y));
    }
    c.sort_unstable();

    let mut it = 0;
    let mut ans = 0;
    for (t, x, y) in c {
        while v[it].0 <= t { it += 1; }
        let t1 = (x - v[it-1].1).pow(2) + (y - v[it-1].2).pow(2);
        let t2 = (x - v[it].1).pow(2) + (y - v[it].2).pow(2);
        if t1 > (t - v[it-1].0).pow(2) || t2 > (v[it].0 - t).pow(2) { ans += 1; }
    }
    println!("{}", ans);
}