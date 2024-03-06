// BOJ 27926 [Quartet]
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
    let mut p = input(7444444);
    let (n, m) = (ptr(&mut p) as usize, ptr(&mut p) as usize);
    let mut top = vec![[(0u32, 0u32); 3]; 200000];
    let (mut x, mut y) = (0, 0);
    for _ in 0..m {
        let (i, j, v) = (ptr(&mut p) as usize-1, ptr(&mut p) as usize-1, ptr(&mut p));
        if x < v { y = x; x = v; } else if y < v { y = v; }
        if top[i][0].0 < v { top[i][2] = top[i][1]; top[i][1] = top[i][0]; top[i][0] = (v, j as u32); }
        else if top[i][1].0 < v { top[i][2] = top[i][1]; top[i][1] = (v, j as u32); }
        else if top[i][2].0 < v { top[i][2] = (v, j as u32); }
        if top[j][0].0 < v { top[j][2] = top[j][1]; top[j][1] = top[j][0]; top[j][0] = (v, i as u32); }
        else if top[j][1].0 < v { top[j][2] = top[j][1]; top[j][1] = (v, i as u32); }
        else if top[j][2].0 < v { top[j][2] = (v, i as u32); }
    }

    let mut ans = 0;
    for i in 0..n as u32 {
        for &(v, j) in &top[i as usize] {
            if i < j { continue; }
            for &(w, k) in &top[i as usize] {
                if j == k { continue; }
                for &(x, l) in &top[j as usize] {
                    if l == i || l == k { continue; }
                    ans = ans.max(v + w + x);
                }
            }
        }
    }
    println!("{}", ans.max(x + y));
}