use std::io::BufRead;
pub fn main() {
    let (mut x, mut y) = (0, 1);
    std::io::stdin().lock().lines().skip(1).next().unwrap().unwrap().chars().for_each(|c| {
        if c == 'C' { x += 1 } else { y += 1 }
    });
    println!("{}", x/y + if x%y > 0 { 1 } else { 0 });
}
