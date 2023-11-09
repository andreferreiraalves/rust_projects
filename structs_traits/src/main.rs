fn main() {
    let mut grapes = Grapes::new(5);
    grapes.bite();

    println!("{:?}", grapes);

    grapes.bite();

    println!("{:?}", grapes);
}

trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)]
struct Grapes {
    amount_left: u32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}

impl Grapes {
    fn new(initial_grapes: u32) -> Self {
        Self {
            amount_left: initial_grapes,
        }
    }
}
