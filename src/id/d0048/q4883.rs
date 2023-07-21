use std::io::{self, Read};
pub fn main() {
    let mut s = String::new();
    io::stdin().lock().read_to_string(&mut s).unwrap();
    let mut it = s.split_ascii_whitespace();
    let mut t = 0;
    loop {
        let n = it.next().unwrap().parse().unwrap();
        if n == 0 { break; }
        let (_, b, c) = (
            it.next().unwrap(),
            it.next().unwrap().parse::<i32>().unwrap(),
            it.next().unwrap().parse::<i32>().unwrap());
        let (mut da, mut db, mut dc) = (0x3f3f3f3f, b, b+c);
        for _ in 1..n {
            da = da.min(db);
            dc = db.min(dc);
            db = da.min(dc);
            let ta = it.next().unwrap().parse::<i32>().unwrap() + da;
            let tb = it.next().unwrap().parse::<i32>().unwrap() + db.min(ta);
            let tc = it.next().unwrap().parse::<i32>().unwrap() + dc.min(tb);
            (da, db, dc) = (ta, tb, tc);
        }
        t += 1;
        println!("{}. {}", t, db);
    }
}
