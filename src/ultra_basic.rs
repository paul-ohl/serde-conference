use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Bonjour {
    foo: u32,
    bar: String,
}

fn main() {
    println!("Hello, world!");
}
