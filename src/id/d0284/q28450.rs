// BOJ 28450 [Conventional Deadlift]

// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = u64; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}

pub fn main() {
    let mut p = input(3777777);
    let (n, m) = (ptr(&mut p) as usize, ptr(&mut p) as usize);

    let mut d = [0; 1000];
    let mut t = [0; 1000];
    d[0] = ptr(&mut p);
    for i in 1..m { d[i] = d[i-1] + ptr(&mut p); }
    for i in 1..n {
        let (d, t) = if i & 1 == 1 { (&mut d, &mut t) } else { (&mut t, &mut d) };
        t[0] = d[0] + ptr(&mut p);
        for j in 1..m {
            let x = ptr(&mut p);
            t[j] = d[j].min(t[j-1]) + x;
        }
    }

    let v = if n & 1 == 0 { t[m-1] } else { d[m-1] };
    let h = ptr(&mut p);
    if v > h { println!("NO"); }
    else { println!("YES\n{}", v); }
}