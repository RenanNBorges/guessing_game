use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe um numero de 1 a 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Por favor, insira um numero.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Parabéns, você ganhou!");
                break;
            }
        }
    }
}

