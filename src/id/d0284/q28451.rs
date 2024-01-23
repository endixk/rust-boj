// BOJ 28451 [Mosquito Killer]

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
    let mut p = input(1 << 21);
    let (n, l) = (ptr(&mut p) as usize, ptr(&mut p) as usize);
    let (a, b) = (ptr(&mut p) as usize, ptr(&mut p));
    let mut d = [0; 100000];
    (0..n).for_each(|i| d[i] = ptr(&mut p));
    let mut h = [0; 100012];
    for &i in &d[..n] { h[i as usize] = h[i as usize].max(ptr(&mut p)); }

    let (mut i, mut c) = (0, 0);
    while i < l {
        let mut mv = h[i+1] == 0;
        if mv {
            for j in 0..=a {
                if h[i+2+j] > b * j as u32 { mv = false; break; }
            }
        }
        if mv { i += 2; continue; }

        for j in 0..=a {
            if h[i+j] > b * j as u32 { println!("-1"); return; }
            h[i+j] = h[i+j].saturating_sub(b);
        }
        i += 1; c += 1;
    }
    println!("{}", l + c);
}