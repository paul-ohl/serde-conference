#![allow(unused)]

use serde::Serialize;

mod pobl;
use pobl::Serializer;

fn main() {
    let sim = Sim {
        name: "Poupidou".to_string(),
        age: 24,
        is_alive: true,
        gender: Gender::Other("Non-binary".to_string()),
        stuff: vec![String::from("Baguette"), String::from("Croissant")],
    };

    // Serialize the data
    let sim_serialized = pobl::to_string(&sim).unwrap();
    println!("Serialized data:\n{}", sim_serialized);
}

#[derive(Serialize)]
struct Sim {
    name: String,
    age: u8, // max age is 255
    is_alive: bool,
    gender: Gender,
    stuff: Vec<String>,
}

#[derive(Serialize)]
enum Gender {
    Male,
    Female,
    Other(String),
}
