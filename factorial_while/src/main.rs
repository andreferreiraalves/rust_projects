use std::io;

fn main() {
    let mut value_typed = String::new();

    println!("Type a number:");

    io::stdin()
        .read_line(&mut value_typed)
        .expect("Error on read value");

    let mut factorial = 1;
    let mut value = value_typed.trim().parse::<i32>().unwrap();

    while value > 1 {
        factorial = factorial * value;

        value = value - 1;
    }

    println!("The factorial number is {}", factorial);
}
