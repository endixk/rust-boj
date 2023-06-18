// BOJ 5373 [Rubik's Cube]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

struct Cube {
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let mut cube = Cube::new();
        for _ in 0..next(&mut it) {
            let cmd = next::<String>(&mut it);
            cube.rotate(cmd.chars().nth(0).unwrap(), cmd.chars().nth(1).unwrap());
        }
        for i in 0..3 {
            for j in 0..3 {
                write!(so, "{}", cube.u[i][j]).unwrap();
            }
            writeln!(so).unwrap();
        }
    }
}
