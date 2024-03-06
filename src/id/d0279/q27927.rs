// BOJ 27927 [Cherry Blossom Ending]
// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = u32; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
struct SegTree {
    n: usize,
    l: Vec<u32>, r: Vec<u32>, c: Vec<u32>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            n: n.next_power_of_two(),
            l: vec![0; n.next_power_of_two() << 1],
            r: vec![0; n.next_power_of_two() << 1],
            c: vec![0; n.next_power_of_two() << 1],
        }
    }
    fn update(&mut self, mut i: usize, x: bool) {
        i |= self.n;
        if x { self.l[i] = 1; self.r[i] = 1; self.c[i] = 1; }
        else { self.l[i] = 0; self.r[i] = 0; self.c[i] = 0; }
        let mut sz = 1;
        while i > 1 {
            i >>= 1;
            let (x, y) = (i<<1, i<<1|1);
            self.l[i] = if self.c[x] == sz { self.c[x] + self.l[y] } else { self.l[x] };
            self.r[i] = if self.c[y] == sz { self.c[y] + self.r[x] } else { self.r[y] };
            self.c[i] = self.c[x].max(self.c[y]).max(self.r[x] + self.l[y]);
            sz <<= 1;
        }
    }
}
pub fn main() {
    let mut p = input(2200022);
    let (n, m) = (ptr(&mut p) as usize, ptr(&mut p) as usize);
    let mut v = Vec::<(usize, bool, usize)>::with_capacity(n<<1);
    for i in 0..n {
        let (s, e) = (ptr(&mut p) as usize, ptr(&mut p) as usize);
        v.push((s, true, i)); v.push((e+1, false, i));
    }
    v.sort_unstable();

    let (mut max, mut cnt, mut prv) = (0, 0, 1);
    let mut seg = SegTree::new(n);
    for (x, b, i) in v {
        if max == seg.c[1] { cnt += x - prv; }
        seg.update(i, b);
        if max < seg.c[1] { max = seg.c[1]; cnt = 0; }
        prv = x;
    }
    if max == seg.c[1] { cnt += m + 1 - prv; }
    println!("{} {}", max, cnt);
}