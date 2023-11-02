use std::io::{self, BufRead};
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut p, mut q) = (1, 0);
    for c in io::stdin().lock().lines().next().unwrap()?.chars() {
        (p, q) = match c.is_lowercase() {
            true =>  ((p + 2).min(q + 2), (p + 2).min(q + 1)),
            false => ((p + 1).min(q + 2), (p + 2).min(q + 2))
        };
    }
    println!("{}", p.min(q));
    Ok(())
}