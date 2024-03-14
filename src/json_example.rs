#![allow(unused)]

fn main() {
    let sim = Sim {
        name: "Sim".to_string(),
        age: 24,
        is_alive: true,
        gender: Gender::Other("Non-binary".to_string()),
    };

    // Serialize the data
    let sim_serialized = serde_json::to_string_pretty(&sim).unwrap();
    println!("Serialized data:\n{}", sim_serialized);
}

#[derive(serde::Serialize)]
struct Sim {
    name: String,
    age: u8, // max age is 255
    is_alive: bool,
    gender: Gender,
}

#[derive(serde::Serialize)]
enum Gender {
    Male,
    Female,
    Other(String),
}
