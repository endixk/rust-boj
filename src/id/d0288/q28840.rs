use std::io::BufRead;
pub fn main() {
    let v = std::io::stdin().lock().lines().take(2).map(|x| x.unwrap()).collect::<Vec<_>>();
    let x = v[0].split(":").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let y = v[1].split(":").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let (x, y) = (x[0] * 60 + x[1], y[0] * 60 + y[1]);
    let t = 24*60 - x + y;
    println!("{:02}:{:02}", t/60, t%60);
}
