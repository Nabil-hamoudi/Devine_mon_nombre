use std::io;
use rand::{thread_rng, Rng};

#[derive(PartialEq, Eq)]
enum Orderin {
    Greater,
    Lesser,
    Equal
}

fn order(secret: u8, input: &String) -> Orderin {
    let input : u8 = input.trim().parse().unwrap();
    if input == secret {Orderin::Equal}
    else if input > secret {Orderin::Greater}
    else {Orderin::Lesser}
}


fn display(ord: &Orderin){
    let message = match ord {
        Orderin::Equal => "Bon nombre bravo !",
        Orderin::Greater => "Trop haut !",
        Orderin::Lesser => "Trop bas !"
    };
    println!("{message}")
}

fn main() {
    println!("Devine mon nombre!");
    let num : u8 = thread_rng().gen_range(1..101);
    loop {
        let mut mess = String::new();
        let _ = io::stdin().read_line(&mut mess);
        let etat = order(num, &mess);
        display(&etat);
        if etat == Orderin::Equal {break}
        }
}


