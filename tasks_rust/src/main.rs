use std::cmp::Ordering;

fn main() {
    // count_for();
    // count_down_while();

    // let numbers = vec![10, 200, 30, 5, 2, 33, 90];
    // get_great_number(&numbers);

    // CheckPrimer number
    let number = 13;
    let result = is_primer(number);

    println!("O número {} é primo {}", number, result);
}

fn count_for() {
    println!("Counter using for");
    for counter in 1..101 {
        println!("Counter2 {}", counter);
    }
}

fn count_down_while() {
    let mut counter = 100;

    println!("Counter using while");

    while counter >= 1 {
        println!("Counter {}", counter);
        counter -= 1;
    }
}

fn get_great_number(vector: &Vec<i32>) {
    // match vector.iter().max() {
    //     Some(max_value) => println!("The greater value is {}", max_value),
    //     None => println!("Vector is empty"),
    // }

    let mut max_number = 0;
    for number in vector {
        if number >= &max_number {
            max_number = *number;
        }
    }

    println!("The greather number is {}", max_number);
}

fn is_primer(number: i32) -> bool {
    false
}
