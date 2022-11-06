use std::io;
use rand::{thread_rng, Rng};

#[derive(PartialEq, Eq)]
enum Orderin {
    Greater,
    Lesser,
    Equal,
    Other
}

fn order(secret: u8, input: &String) -> Orderin {
    match input.trim().parse::<u8>() {
        Ok(val) => {if val <= 0 {Orderin::Other}
                   else if val > 100 {Orderin::Other}
                   else if val == secret {Orderin::Equal}
                   else if val > secret {Orderin::Greater}
                   else {Orderin::Lesser}},
        Err(_) => Orderin::Other
    }
}


fn display(ord: &Orderin){
    let message = match ord {
        Orderin::Equal => "Bon nombre bravo !",
        Orderin::Greater => "Trop haut !",
        Orderin::Lesser => "Trop bas !",
        Orderin::Other => "Entrée invalide veuillez entrée un nombre entre 1 et 100"
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


