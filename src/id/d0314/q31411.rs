// BOJ 31411 [Contest Hosting]
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
    let mut p = input(1 << 23);
    let (n, k) = (ptr(&mut p), ptr(&mut p));
    let nk = (n as usize) * (k as usize);
    let mut a = Vec::<(u32, u32)>::with_capacity(nk);
    for i in 0..n {
        for _ in 0..k { a.push((ptr(&mut p), i)); }
    }
    a.sort_unstable();

    let mut cnt = vec![0; n as usize];
    let mut ans = 1<<30;
    let (mut l, mut r, mut c) = (0, 0, 0);
    loop {
        while r < nk && c < n {
            if cnt[a[r].1 as usize] == 0 { c += 1; }
            cnt[a[r].1 as usize] += 1;
            r += 1;
        }
        if c < n { break; }
        ans = ans.min(a[r - 1].0 - a[l].0);
        cnt[a[l].1 as usize] -= 1;
        if cnt[a[l].1 as usize] == 0 { c -= 1; }
        l += 1;
    }
    println!("{}", ans);
}