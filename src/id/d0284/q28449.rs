// BOJ 28449 [Who Will Win?]

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
    let (n, m) = (ptr(&mut p) as usize, ptr(&mut p) as usize);

    let mut c = [0u32; 100001];
    for _ in 0..n { c[ptr(&mut p) as usize] += 1; }
    let mut a = [0u32; 100001];
    let mut i = 0;
    for (x, &y) in c.iter().enumerate() {
        for _ in 0..y { a[i] = x as u32; i += 1; }
    }

    c = [0u32; 100001];
    for _ in 0..m { c[ptr(&mut p) as usize] += 1; }
    let mut b = [0u32; 100001];
    let mut i = 0;
    for (x, &y) in c.iter().enumerate() {
        for _ in 0..y { b[i] = x as u32; i += 1; }
    }

    let (mut w, mut d, mut l) = (0, 0, 0);
    let (mut i, mut j) = (0, 0);
    for &x in b[..m].iter() {
        while i < n && a[i] < x { i += 1; }
        while j < n && a[j] <= x { j += 1; }
        l += i; d += j - i; w += n - j;
    }
    println!("{} {} {}", w, l, d);
}