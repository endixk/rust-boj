use std::io::Read;
pub fn main() {
    let mut s = String::new();
    std::io::stdin().lock().read_to_string(&mut s).unwrap();
    let v = s.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut a = (1..=v[0]).collect::<Vec<_>>();
    for i in 0..v[1] { a[v[i*2+2]-1..v[i*2+3]].reverse(); }
    a.iter().for_each(|x| print!("{} ", x));
}
