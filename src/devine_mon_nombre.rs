use std::io;

fn main() {
    println!("Devine mon nombre!");
    let mut mess = String::new();
    let num = io::stdin().read_line(&mut mess);
    println!("{}", num.unwrap());
    println!("{mess}")
}
