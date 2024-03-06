// BOJ 27923 [How-many-burgers can you eat at once?]
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
    let mut p = input(1 << 22);
    let (n, k, l) = (ptr(&mut p) as usize, ptr(&mut p) as usize, ptr(&mut p) as usize);
    let mut m = (0..n).map(|_| ptr(&mut p)).collect::<Vec<_>>();
    m.sort_unstable();

    let mut a = [0i32; 200_000];
    for _ in 0..k {
        let t = (ptr(&mut p) - 1) as usize;
        a[t] += 1;
        if t + l < n { a[t + l] -= 1; }
    }

    let mut c = [0i32; 64];
    let mut x = 0;
    for i in 0..n {
        x += a[i];
        if x < 64 { c[x as usize] += 1; }
    }

    let mut ans = 0;
    let mut j = 0;
    for i in 0..64 {
        for _ in 0..c[i] { ans += m[j].checked_shr(i as u32).unwrap_or(0) as u64; j += 1; }
    }
    println!("{}", ans);
}