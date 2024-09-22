// BOJ 6242 [City Skyline]
pub fn main() {
    let p = &mut input(2222222);
    let mut v = vec![];
    for _ in 0..u32(p) {
        let (a, b, h) = (u32(p), u32(p), u32(p) as i32);
        v.push((a, h)); v.push((b, -h));
    }
    v.sort_unstable();

    let (mut ans, mut prx, mut prh) = (0, 0, 0);
    let mut map = std::collections::BTreeMap::new();
    for (x, h) in v {
        ans += (x - prx) as i64 * prh as i64;
        if h > 0 {
            *map.entry(h).or_insert(0) += 1;
        } else {
            if let Some(v) = map.get_mut(&-h) {
                *v -= 1;
                if *v == 0 { map.remove(&-h); }
            }
        }
        prx = x; prh = *map.keys().next_back().unwrap_or(&0);
    }
    println!("{}", ans);
}
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn u32(p: &mut *const u8) -> u32 { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as u32 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}