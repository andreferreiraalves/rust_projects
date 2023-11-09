use std::io;

const ERROR_ON_READ: &str = "error on read value";

fn main() {
    let mut students_in_recovery = 0;

    let mut counter = read_message("Enter number of grades");

    while counter > 0 {
        let grade = read_message("Enter the student's grade");

        if (grade >= 3) && (grade <= 6) {
            students_in_recovery += 1;
        }

        counter -= 1;
    }

    println!(
        "The number of students in recoverry is {}",
        students_in_recovery
    );
}

fn read_message(message: &str) -> i32 {
    let mut value_typed = String::new();

    println!("{}", message);

    io::stdin()
        .read_line(&mut value_typed)
        .expect(ERROR_ON_READ);

    value_typed.trim().parse::<i32>().unwrap()
}
