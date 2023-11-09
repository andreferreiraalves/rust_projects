fn main() {
    println!("Hello, world!");
    let rectangle = Rectangle {
        width: 10,
        length: 10,
    };

    println!("Total area {}", rectangle.area());
    println!("Retangulo {:?}", rectangle);
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}
