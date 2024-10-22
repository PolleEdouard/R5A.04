use std::io;

fn loop_in_time(n: i32) {
    for i in 0..n {
        println!("{} Abracadabra", i+1);
    }
}

fn main() {
    // Créer une nouvelle variable mutable pour stocker l'entrée utilisateur
    let mut input = String::new();

    // Lire l'entrée utilisateur
    io::stdin().read_line(&mut input).expect("Erreur de lecture");

    // Convertir l'entrée en i32
    let n: i32 = input.trim().parse().expect("Veuillez entrer un nombre valide");

    loop_in_time(n);
}
