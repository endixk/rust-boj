// BOJ 6596 [Globetrotter]
use std::f64::consts::PI;
const ER: f64 = 6378.0;
struct Loc { lat: f64, lon: f64 }
fn dist(lx: &Loc, ly: &Loc) -> f64 {
    ER * (
        lx.lat.sin() * ly.lat.sin() +
        lx.lat.cos() * ly.lat.cos() * (lx.lon - ly.lon).cos()
    ).acos()
}
pub fn main() { read();
    let mut names = std::collections::HashMap::new();
    let mut locs = Vec::new();
    loop {
        let city = next::<String>();
        if city == "#" { break; }
        let (lat, lon) = (next::<f64>().to_radians(), next::<f64>().to_radians());
        names.insert(city, locs.len());
        locs.push(Loc { lat, lon });
    }
    loop {
        let (city1, city2) = (next::<String>(), next::<String>());
        if city1 == "#" && city2 == "#" { break; }
        if !names.contains_key(&city1) || !names.contains_key(&city2) {
            println!("{} - {}\n{}\n", city1, city2, "Unknown");
        } else {
            let d = dist(&locs[names[&city1]], &locs[names[&city2]]);
            println!("{} - {}\n{:.0} km\n", city1, city2, d);
        }
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