// BOJ 22266 [Crisis at the Wedding]
// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = u32; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
fn solve(a: &mut Vec<i64>, n: usize) -> i64 {
    let mut ret = 0;
    let mut vis = vec![false; n];
    let (mut i, mut x, mut c) = (0, 0, 0);
    while c < n {
        if a[i] > 0 {
            x += a[i]; a[i] = 0; vis[i] = true; c += 1;
        } else if a[i] == 0 {
            if !vis[i] {
                vis[i] = true; c += 1;
            }
        } else {
            if -a[i] < x {
                x += a[i]; a[i] = 0;
            } else if -a[i] > x {
                a[i] += x; x = 0;
            } else {
                a[i] = 0; x = 0; vis[i] = true; c += 1;
            }
        }
        i = (i + 1) % n;
        ret += x;
    }
    ret
}
pub fn main() {
    let mut p = input(500500);
    let n = ptr(&mut p) as usize;
    let mut a = vec![0; n];
    let mut s = 0;
    for i in 0..n {
        let x = ptr(&mut p) as i64;
        a[i] = x; s += x;
    }

    a.iter_mut().for_each(|x| *x -= s / n as i64);
    let mut k = solve(&mut a.clone(), n);
    a.reverse();
    k = k.min(solve(&mut a, n));
    println!("{}", k);
}