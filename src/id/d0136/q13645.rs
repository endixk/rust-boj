// BOJ 13645 [Ciclo de Rubik]
#[derive(Eq, PartialEq)] struct Cube {
    u: [[char; 3]; 3],
    d: [[char; 3]; 3],
    f: [[char; 3]; 3],
    b: [[char; 3]; 3],
    l: [[char; 3]; 3],
    r: [[char; 3]; 3],
}
impl Cube {
    fn new() -> Self {
        Cube {
            u: [['w'; 3]; 3],
            d: [['y'; 3]; 3],
            f: [['r'; 3]; 3],
            b: [['o'; 3]; 3],
            l: [['g'; 3]; 3],
            r: [['b'; 3]; 3],
        }
    }
    fn ucw(&mut self) { // up, clockwise
        self.u = [
            [self.u[2][0], self.u[1][0], self.u[0][0]],
            [self.u[2][1], self.u[1][1], self.u[0][1]],
            [self.u[2][2], self.u[1][2], self.u[0][2]]
        ];
        let tmp = self.f[0];
        self.f[0] = self.r[0];
        self.r[0] = self.b[0];
        self.b[0] = self.l[0];
        self.l[0] = tmp;
    }
    fn ucc(&mut self) { // up, counter-clockwise
        self.u = [
            [self.u[0][2], self.u[1][2], self.u[2][2]],
            [self.u[0][1], self.u[1][1], self.u[2][1]],
            [self.u[0][0], self.u[1][0], self.u[2][0]]
        ];
        let tmp = self.f[0];
        self.f[0] = self.l[0];
        self.l[0] = self.b[0];
        self.b[0] = self.r[0];
        self.r[0] = tmp;
    }
    fn dcw(&mut self) { // down, clockwise
        self.d = [
            [self.d[2][0], self.d[1][0], self.d[0][0]],
            [self.d[2][1], self.d[1][1], self.d[0][1]],
            [self.d[2][2], self.d[1][2], self.d[0][2]]
        ];
        let tmp = self.f[2];
        self.f[2] = self.l[2];
        self.l[2] = self.b[2];
        self.b[2] = self.r[2];
        self.r[2] = tmp;
    }
    fn dcc(&mut self) { // down, counter-clockwise
        self.d = [
            [self.d[0][2], self.d[1][2], self.d[2][2]],
            [self.d[0][1], self.d[1][1], self.d[2][1]],
            [self.d[0][0], self.d[1][0], self.d[2][0]]
        ];
        let tmp = self.f[2];
        self.f[2] = self.r[2];
        self.r[2] = self.b[2];
        self.b[2] = self.l[2];
        self.l[2] = tmp;
    }
    fn fcw(&mut self) { // front, clockwise
        self.f = [
            [self.f[2][0], self.f[1][0], self.f[0][0]],
            [self.f[2][1], self.f[1][1], self.f[0][1]],
            [self.f[2][2], self.f[1][2], self.f[0][2]]
        ];
        let tmp = self.u[2];
        self.u[2] = [self.l[2][2], self.l[1][2], self.l[0][2]];
        (self.l[2][2], self.l[1][2], self.l[0][2]) = (self.d[0][2], self.d[0][1], self.d[0][0]);
        self.d[0] = [self.r[2][0], self.r[1][0], self.r[0][0]];
        (self.r[0][0], self.r[1][0], self.r[2][0]) = (tmp[0], tmp[1], tmp[2]);
    }
    fn fcc(&mut self) { // front, counter-clockwise
        self.f = [
            [self.f[0][2], self.f[1][2], self.f[2][2]],
            [self.f[0][1], self.f[1][1], self.f[2][1]],
            [self.f[0][0], self.f[1][0], self.f[2][0]]
        ];
        let tmp = self.u[2];
        self.u[2] = [self.r[0][0], self.r[1][0], self.r[2][0]];
        (self.r[0][0], self.r[1][0], self.r[2][0]) = (self.d[0][2], self.d[0][1], self.d[0][0]);
        self.d[0] = [self.l[0][2], self.l[1][2], self.l[2][2]];
        (self.l[2][2], self.l[1][2], self.l[0][2]) = (tmp[0], tmp[1], tmp[2]);
    }
    fn bcw(&mut self) { // back, clockwise
        self.b = [
            [self.b[2][0], self.b[1][0], self.b[0][0]],
            [self.b[2][1], self.b[1][1], self.b[0][1]],
            [self.b[2][2], self.b[1][2], self.b[0][2]]
        ];
        let tmp = self.u[0];
        self.u[0] = [self.r[0][2], self.r[1][2], self.r[2][2]];
        (self.r[0][2], self.r[1][2], self.r[2][2]) = (self.d[2][2], self.d[2][1], self.d[2][0]);
        (self.d[2][2], self.d[2][1], self.d[2][0]) = (self.l[2][0], self.l[1][0], self.l[0][0]);
        (self.l[2][0], self.l[1][0], self.l[0][0]) = (tmp[0], tmp[1], tmp[2]);
    }
    fn bcc(&mut self) { // back, counter-clockwise
        self.b = [
            [self.b[0][2], self.b[1][2], self.b[2][2]],
            [self.b[0][1], self.b[1][1], self.b[2][1]],
            [self.b[0][0], self.b[1][0], self.b[2][0]]
        ];
        let tmp = self.u[0];
        self.u[0] = [self.l[2][0], self.l[1][0], self.l[0][0]];
        (self.l[2][0], self.l[1][0], self.l[0][0]) = (self.d[2][2], self.d[2][1], self.d[2][0]);
        (self.d[2][2], self.d[2][1], self.d[2][0]) = (self.r[0][2], self.r[1][2], self.r[2][2]);
        (self.r[0][2], self.r[1][2], self.r[2][2]) = (tmp[0], tmp[1], tmp[2]);

    }
    fn lcw(&mut self) { // left, clockwise
        self.l = [
            [self.l[2][0], self.l[1][0], self.l[0][0]],
            [self.l[2][1], self.l[1][1], self.l[0][1]],
            [self.l[2][2], self.l[1][2], self.l[0][2]]
        ];
        let tmp = [self.u[0][0], self.u[1][0], self.u[2][0]];
        (self.u[0][0], self.u[1][0], self.u[2][0]) = (self.b[2][2], self.b[1][2], self.b[0][2]);
        (self.b[2][2], self.b[1][2], self.b[0][2]) = (self.d[0][0], self.d[1][0], self.d[2][0]);
        (self.d[0][0], self.d[1][0], self.d[2][0]) = (self.f[0][0], self.f[1][0], self.f[2][0]);
        (self.f[0][0], self.f[1][0], self.f[2][0]) = (tmp[0], tmp[1], tmp[2]);
    }
    fn lcc(&mut self) { // left, counter-clockwise
        self.l = [
            [self.l[0][2], self.l[1][2], self.l[2][2]],
            [self.l[0][1], self.l[1][1], self.l[2][1]],
            [self.l[0][0], self.l[1][0], self.l[2][0]]
        ];
        let tmp = [self.u[0][0], self.u[1][0], self.u[2][0]];
        (self.u[0][0], self.u[1][0], self.u[2][0]) = (self.f[0][0], self.f[1][0], self.f[2][0]);
        (self.f[0][0], self.f[1][0], self.f[2][0]) = (self.d[0][0], self.d[1][0], self.d[2][0]);
        (self.d[0][0], self.d[1][0], self.d[2][0]) = (self.b[2][2], self.b[1][2], self.b[0][2]);
        (self.b[2][2], self.b[1][2], self.b[0][2]) = (tmp[0], tmp[1], tmp[2]);
    }
    fn rcw(&mut self) { // right, clockwise
        self.r = [
            [self.r[2][0], self.r[1][0], self.r[0][0]],
            [self.r[2][1], self.r[1][1], self.r[0][1]],
            [self.r[2][2], self.r[1][2], self.r[0][2]]
        ];
        let tmp = [self.u[0][2], self.u[1][2], self.u[2][2]];
        (self.u[0][2], self.u[1][2], self.u[2][2]) = (self.f[0][2], self.f[1][2], self.f[2][2]);
        (self.f[0][2], self.f[1][2], self.f[2][2]) = (self.d[0][2], self.d[1][2], self.d[2][2]);
        (self.d[0][2], self.d[1][2], self.d[2][2]) = (self.b[2][0], self.b[1][0], self.b[0][0]);
        (self.b[2][0], self.b[1][0], self.b[0][0]) = (tmp[0], tmp[1], tmp[2]);
    }
    fn rcc(&mut self) { // right, counter-clockwise
        self.r = [
            [self.r[0][2], self.r[1][2], self.r[2][2]],
            [self.r[0][1], self.r[1][1], self.r[2][1]],
            [self.r[0][0], self.r[1][0], self.r[2][0]]
        ];
        let tmp = [self.u[0][2], self.u[1][2], self.u[2][2]];
        (self.u[0][2], self.u[1][2], self.u[2][2]) = (self.b[2][0], self.b[1][0], self.b[0][0]);
        (self.b[2][0], self.b[1][0], self.b[0][0]) = (self.d[0][2], self.d[1][2], self.d[2][2]);
        (self.d[0][2], self.d[1][2], self.d[2][2]) = (self.f[0][2], self.f[1][2], self.f[2][2]);
        (self.f[0][2], self.f[1][2], self.f[2][2]) = (tmp[0], tmp[1], tmp[2]);
    }
    fn rotate(&mut self, face: char, dir: char) {
        match face {
            'U' => { if dir == '+' { self.ucw(); } else { self.ucc(); } }
            'D' => { if dir == '+' { self.dcw(); } else { self.dcc(); } }
            'F' => { if dir == '+' { self.fcw(); } else { self.fcc(); } }
            'B' => { if dir == '+' { self.bcw(); } else { self.bcc(); } }
            'L' => { if dir == '+' { self.lcw(); } else { self.lcc(); } }
            'R' => { if dir == '+' { self.rcw(); } else { self.rcc(); } }
            _ => {}
        }
    }
}
const RDC: [&str; 12] = [
    "Uu", "Dd", "Ff", "Bb", "Ll", "Rr",
    "UUUU", "DDDD", "FFFF", "BBBB", "LLLL", "RRRR",
];
pub fn main() { read();
    while peek() {
        let mut seq = next::<String>();
        // reduce sequence
        'x: loop {
            for &r in RDC.iter() {
                if seq.contains(r) {
                    seq = seq.replace(r, "");
                    continue 'x;
                }
            }
            break;
        }
        if seq.is_empty() { println!("1"); continue; }

        let ini = Cube::new();
        let mut cube = Cube::new();
        let mut ans = 1;
        loop {
            for c in seq.chars() {
                cube.rotate(c.to_ascii_uppercase(), if c.is_ascii_uppercase() { '+' } else { '-' });
            }
            if cube == ini { break; }
            ans += 1;
        }
        println!("{}", ans);
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, iter::Peekable, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<Peekable<SplitAsciiWhitespace>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace().peekable());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}
fn peek() -> bool { unsafe { IT.as_mut().unwrap().peek().is_some() } }