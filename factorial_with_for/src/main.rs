use std::io;

fn main() {
    let mut typed_number = String::new();
    let mut factorial = 1;

    println!("Type a factorial number");

    io::stdin()
        .read_line(&mut typed_number)
        .expect("Error on read value");

    let mut number = typed_number.trim().parse::<u32>().unwrap();

    while number > 1 {
        factorial = factorial * number;

        number -= 1;
    }

    println!("Result {}", factorial);
}
