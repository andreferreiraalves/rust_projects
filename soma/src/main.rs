use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut total = 0;
    let mut value_input = String::new();

    println!("Type a number like 123456");

    io::stdin()
        .read_line(&mut value_input)
        .expect("Error on read value !");

    let mut int_from_input = convert_to_int(&value_input);

    while int_from_input != 0 {
        total += int_from_input % 10;
        int_from_input /= 10;
    }

    println!("The total value is {}", total);
}
