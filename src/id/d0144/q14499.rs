// BOJ 14499 [Rolling Dice]
#[derive(Default)] struct Dice { f: [u8; 6] } // [top, front, right, left, back, bottom]
impl Dice {
    fn roll(&mut self, d: u8) {
        self.f = match d {
            1 => [self.f[3], self.f[1], self.f[0], self.f[5], self.f[4], self.f[2]], // East
            2 => [self.f[2], self.f[1], self.f[5], self.f[0], self.f[4], self.f[3]], // West
            3 => [self.f[4], self.f[0], self.f[2], self.f[3], self.f[5], self.f[1]], // North
            _ => [self.f[1], self.f[5], self.f[2], self.f[3], self.f[0], self.f[4]], // South
        }
    }
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let (mut x, mut y) = (next::<usize>(), next::<usize>());
    let k = next::<usize>();
    let mut a = vec![vec![0u8; m]; n];
    for i in 0..n { for j in 0..m { a[i][j] = next(); } }

    let mut dice = Dice::default();
    for _ in 0..k {
        let d = next::<u8>();
        let (nx, ny) = match d {
            1 if y+1 < m => (x, y+1),
            2 if y > 0 => (x, y-1),
            3 if x > 0 => (x-1, y),
            4 if x+1 < n => (x+1, y),
            _ => continue,
        };

        (x, y) = (nx, ny);
        dice.roll(d);
        if a[x][y] == 0 { a[x][y] = dice.f[5]; }
        else { dice.f[5] = a[x][y]; a[x][y] = 0; }
        println!("{}", dice.f[0]);
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}