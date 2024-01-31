// BOJ 6570 [Equidistance]

use std::f64::consts::PI;

// polar to cartesian coordinates
const ER: f64 = 6378.0; // Earth radius
fn p2c(lat: f64, lon: f64) -> (f64, f64, f64) {
    (
        ER * lat.to_radians().cos() * lon.to_radians().cos(),
        ER * lat.to_radians().cos() * lon.to_radians().sin(),
        ER * lat.to_radians().sin(),
    )
}

pub fn main() { read();
    let mut map = std::collections::HashMap::<String,usize>::new();
    let mut crd = vec![];

    for i in 0.. {
        let city = next::<String>();
        if city == "#" { break; }
        let (x, y, z) = p2c(next::<f64>(), next::<f64>());
        map.insert(city, i);
        crd.push((x, y, z));
    }

    loop {
        let c1 = next::<String>();
        if c1 == "#" { break; }
        let (c2, c3) = (next::<String>(), next::<String>());
        let (i, j, k) = if !map.contains_key(&c1) || !map.contains_key(&c2) || !map.contains_key(&c3) {
            println!("{c3} is ? km off {c1}/{c2} equidistance.");
            continue;
        } else { (map[&c1], map[&c2], map[&c3]) };

        // normal vector of plane
        let (vx, vy, vz) = (crd[i].0 - crd[j].0, crd[i].1 - crd[j].1, crd[i].2 - crd[j].2);
        if vx.abs() < 1e-9 && vy.abs() < 1e-9 && vz.abs() < 1e-9 {
            println!("{c3} is 0 km off {c1}/{c2} equidistance.");
            continue;
        }

        let a = ((vx * crd[k].0 + vy * crd[k].1 + vz * crd[k].2) / (vx * vx + vy * vy + vz * vz).sqrt() / ER).acos();
        let d = ER * (0.5 * PI - a);
        println!("{c3} is {:.0} km off {c1}/{c2} equidistance.", d.abs());
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}