// BOJ 10075 [Friend]
pub fn main() {
    let p = &mut input(1414141);
    let n = u32(p);
    let mut c = (0..n).map(|_| u32(p)).collect::<Vec<_>>();
    let q = (1..n).map(|_| (u32(p), u32(p) as u8)).collect::<Vec<_>>();

    let mut ans = 0;
    for (b, (a, p)) in q.into_iter().enumerate().rev() {
        let (a, b) = (a as usize, b + 1);
        match p {
            1 => c[a] += c[b],
            2 => c[a] = c[a].max(c[b]),
            _ => {
                ans += c[b];
                c[a] = c[a].saturating_sub(c[b]);
            }
        }
    }
    println!("{}", ans + c[0]);
}
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn u32(p: &mut *const u8) -> u32 { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as u32 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}