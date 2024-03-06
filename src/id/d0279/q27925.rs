// BOJ 27925 [Induction]
// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = u32; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}

const INF: i32 = 0x3f3f3f3f;
const D: [[i32; 10]; 10] = [
    [0, 1, 2, 3, 4, 5, 4, 3, 2, 1],
    [1, 0, 1, 2, 3, 4, 5, 4, 3, 2],
    [2, 1, 0, 1, 2, 3, 4, 5, 4, 3],
    [3, 2, 1, 0, 1, 2, 3, 4, 5, 4],
    [4, 3, 2, 1, 0, 1, 2, 3, 4, 5],
    [5, 4, 3, 2, 1, 0, 1, 2, 3, 4],
    [4, 5, 4, 3, 2, 1, 0, 1, 2, 3],
    [3, 4, 5, 4, 3, 2, 1, 0, 1, 2],
    [2, 3, 4, 5, 4, 3, 2, 1, 0, 1],
    [1, 2, 3, 4, 5, 4, 3, 2, 1, 0],
];
pub fn main() {
    let mut p = input(10101);
    let n = ptr(&mut p) as usize;
    let mut dp = [INF; 1<<12];
    let mut tp = [INF; 1<<12];
    dp[0] = 0;
    for t in 0..n {
        let (dp, tp) = if t & 1 == 0 { (&mut dp, &mut tp) } else { (&mut tp, &mut dp) };
        tp.fill(INF);
        let x = ptr(&mut p) as usize;
        for i in 0..10 { for j in 0..10 { for k in 0..10 {
            let u = i<<8 | j<<4 | k;
            if dp[u] == INF { continue; }
            tp[x<<8 | j<<4 | k] = tp[x<<8 | j<<4 | k].min(dp[u] + D[i][x]);
            tp[i<<8 | x<<4 | k] = tp[i<<8 | x<<4 | k].min(dp[u] + D[j][x]);
            tp[i<<8 | j<<4 | x] = tp[i<<8 | j<<4 | x].min(dp[u] + D[k][x]);
        }}}
    }
    let dp = if n & 1 == 0 { &dp } else { &tp };
    println!("{}", dp.iter().min().unwrap());
}