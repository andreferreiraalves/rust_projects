use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    // println!("O número secreto é {}", secret_number);

    println!("Advinhe o número!");

    loop {
        let mut user_guess = String::new();

        println!("Digite o seu palpite entre 1 e 100");

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Erro na leitura do palpite do usuário");

        let guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Número inválido");
                continue;
            }
        };
        // .expect("Erro em converter o palpite do usuário");

        println!("O palpite do usuário foi {}", user_guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
