fn main() {
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();
}

enum Shot {
    Bullseye,
    Hit(f64),
    Miss,
}

impl Shot {
    fn points(self) -> i32 {
        match self {
            Shot::Bullseye => 5,
            Shot::Hit(x) if x < 3.0 => 2,
            Shot::Hit(x) => 1,
            Shot::Miss => 0,
        }
    }
}
