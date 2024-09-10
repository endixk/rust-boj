// BOJ 23353 [Manipulation]
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn u16(p: &mut *const u8) -> u16 { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as u16 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
pub fn main() {
    let p = &mut input(2000022);
    let mut b = [0u8; 1<<20];
    let n = u16(p) as usize;
    for i in 1..=n { for j in 1..=n {
        b[i<<10|j] = u16(p) as u8;
    }}
    let (mut u, mut l, mut ul, mut ur) = ([0u16; 1<<20], [0u16; 1<<20], [0u16; 1<<20], [0u16; 1<<20]);
    for i in 1..=n { for j in 1..=n {
        if b[i<<10|j] != 1 { continue; }
        u[i<<10|j] = u[(i-1)<<10|j] + 1;
        l[i<<10|j] = l[i<<10|j-1] + 1;
        ul[i<<10|j] = ul[(i-1)<<10|j-1] + 1;
        ur[i<<10|j] = ur[(i-1)<<10|j+1] + 1;
    }}
    let (mut d, mut r, mut dl, mut dr) = ([0u16; 1<<20], [0u16; 1<<20], [0u16; 1<<20], [0u16; 1<<20]);
    let mut ans = 0;
    for i in (1..=n).rev() { for j in (1..=n).rev() {
        if b[i<<10|j] != 0 {
            ans = ans.max(u[(i-1)<<10|j] + d[(i+1)<<10|j])
                .max(l[i<<10|j-1] + r[i<<10|j+1])
                .max(ul[(i-1)<<10|j-1] + dr[(i+1)<<10|j+1])
                .max(ur[(i-1)<<10|j+1] + dl[(i+1)<<10|j-1]);
        }
        if b[i<<10|j] != 1 { continue; }
        d[i<<10|j] = d[(i+1)<<10|j] + 1;
        r[i<<10|j] = r[i<<10|j+1] + 1;
        dl[i<<10|j] = dl[(i+1)<<10|j-1] + 1;
        dr[i<<10|j] = dr[(i+1)<<10|j+1] + 1;
    }}
    println!("{}", ans + 1);
}