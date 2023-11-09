use std::{
    sync::mpsc::channel,
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    // let handle = thread::spawn(|| expensive_sum(my_vector));
    let handler = thread::spawn(move || expensive_sum(my_vector));

    for letter in vec!["a", "b", "c", "d", "e"] {
        println!("Main thread: Leatter {}", letter);
        sleep(Duration::new(5, 0));
    }

    // let sum = handler.join().expect("Error on execute thread");
    let sum = handler.join().unwrap();
    println!("the thread sum {}", sum);
}

fn expensive_sum(v: Vec<i32>) -> i32 {
    sleep(Duration::new(5, 0));

    println!("Child thread: just about finished");

    v.into_iter().filter(|&x| x % 2 == 0).map(|x| x * x).sum()
}
