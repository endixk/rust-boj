// Reference: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn u32(p: &mut *const u8) -> u32 { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as u32 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}

fn i32(p: &mut *const u8) -> i32 { unsafe {
    let mut n = 0;
    let neg = if **p == b'-' { *p = p.offset(1); true } else { false };
    while **p & 16 != 0 { n = n * 10 + (**p as i32 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    if neg { -n } else { n }
}}

fn word(p: &mut *const u8) -> &'static str { unsafe {
    let s = *p;
    while **p != 10 { *p = p.offset(1) }
    let r = from_utf8(std::slice::from_raw_parts(s, *p as usize - s as usize)).unwrap();
    *p = p.offset(1);
    r
}}