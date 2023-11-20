// BOJ 30506 [Scissors Scissors Scissors]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};
fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let x = read(&mut si).trim().parse::<u8>()?;
    let mut ans = vec!['0'; 100];
    for i in 0..100 {
        let mut s = vec!['2'; 100];
        s[i] = '0';
        writeln!(so, "? {}", s.into_iter().collect::<String>())?;
        so.flush()?;

        ans[i] = match read(&mut si).trim().parse::<u8>()? {
            y if x+1 == y => '2',
            y if x == y => '0',
            _ => '5',
        };
    }
    writeln!(so, "! {}", ans.into_iter().collect::<String>())?;
    so.flush()?;

    Ok(())
}
