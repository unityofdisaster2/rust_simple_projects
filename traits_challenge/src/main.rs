use std::fmt::Display;

struct Satellite {
    name: String,
    velocity: f64
}

impl Display for Satellite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.name, self.velocity)
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble telescope"),
        velocity: 4.72
    };

    println!("hubble is {}", hubble);
}