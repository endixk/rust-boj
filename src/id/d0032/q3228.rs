// BOJ 3228 [PIZZA]
pub fn main() {
    let p = &mut input(9999);
    let (k, r, m) = (i32(p) as u32, i32(p), i32(p) as usize);
    let v = (0..m).map(|_| (i32(p), i32(p))).collect::<Vec<_>>();
    let n = i32(p) as usize;
    let w = (0..n).map(|_| (i32(p), i32(p), i32(p))).collect::<Vec<_>>();

    let bit = v.iter().map(
        |&(x1, y1)| (0..n).fold(0u128, |acc, j| if (x1 - w[j].0).pow(2) + (y1 - w[j].1).pow(2) <= r.pow(2) { acc | 1 << j } else { acc })
    ).collect::<Vec<_>>();
    let mut ans = 0;
    for b in 0..1u32<<m {
        if b.count_ones() != k { continue; }
        let x = (0..m).fold(0, |acc, i| if b & 1 << i != 0 { acc | bit[i] } else { acc });
        ans = ans.max((0..n).fold(0, |acc, i| if x & 1 << i != 0 { acc + w[i].2 } else { acc }));
    }
    println!("{}", ans);
}
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn i32(p: &mut *const u8) -> i32 { unsafe {
    let mut n = 0;
    let neg = if **p == b'-' { *p = p.offset(1); true } else { false };
    while **p & 16 != 0 { n = n * 10 + (**p as i32 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    if neg { -n } else { n }
}}