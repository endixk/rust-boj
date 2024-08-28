// BOJ 3015 [Oasis 27.08.24]
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn ptr(p: &mut *const u8) -> u32 { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as u32 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
pub fn main() {
    let mut p = input(5500055);
    let mut st = Vec::new();
    let mut ans = 0;
    for _ in 0..ptr(&mut p) {
        let mut h = (ptr(&mut p) as u64)<<20|1;
        while let Some(q) = st.pop() {
            let (x, k) = (q>>20, q&0xfffff);
            if x > h>>20 {
                ans += 1; st.push(q); st.push(h); break;
            } else if x == h>>20 {
                ans += k; h = x<<20|k+1;
            } else {
                ans += k;
            }
        }
        if st.is_empty() { st.push(h); }
    }
    println!("{}", ans);
}