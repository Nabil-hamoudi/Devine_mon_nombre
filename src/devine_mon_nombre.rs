use std::io;

fn main() {
    println!("Devine mon nombre!");
    let mut mess = String::new();
    io::stdin().read_line(&mut mess);
    println!("{mess}")
}
