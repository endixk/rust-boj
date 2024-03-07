// BOJ 31410 [Decontamination]
// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = i32; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    let mut flag = false;
    if **p == b'-' { flag = true; *p = p.offset(1); }
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    if flag { -n } else { n }
}}
pub fn main() {
    let mut p = input(1 << 22);
    let n = ptr(&mut p) as usize;
    let mut a = vec![(0i64, 0i64); 100000];
    for i in 0..n { a[i] = (ptr(&mut p) as i64, ptr(&mut p) as i64); }
    a[..n].sort_unstable();

    let (mut x1, mut y1) = (a[0].1, 0);
    let (mut x2, mut y2) = (a[n-1].1, 0);
    let mut ans = 0;
    for i in 1..n {
        if i == n-1 { ans = x1.min(x2); }
        let (d1, d2) = (a[i].0 - a[i-1].0, a[n-i].0 - a[n-i-1].0);
        let (k1, k2) = (a[i].1, a[n-i-1].1);
        let i = i as i64;
        (x1, y1) = (x1 + i * d1 + k1, (x1 + i * d1).min(y1 + (i-1) * d1 + k1));
        (x2, y2) = (x2 + i * d2 + k2, (x2 + i * d2).min(y2 + (i-1) * d2 + k2));
    }
    println!("{}", ans.min(y1).min(y2));
}