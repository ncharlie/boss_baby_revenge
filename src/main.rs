use boss_baby_revenge::solve;

fn main() {
    use std::io::{stdin,stdout,Write};
    print!("input: ");
    let _ = stdout().flush();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    println!("result: {}",solve(&buffer.trim()));
}
