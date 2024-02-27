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

    // Deserialize the data
    let sim_deserialized: Sim = serde_json::from_str(&sim_serialized).unwrap();
    println!("Serialized data:\n{:?}", sim_deserialized);
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Sim {
    name: String,
    age: u8,
    is_alive: bool,
    gender: Gender,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
enum Gender {
    Male,
    Female,
    Other(String),
}
