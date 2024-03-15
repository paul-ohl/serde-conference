#![allow(unused)]

fn main() {
    let sim = Sim {
        name: "Poupidou".to_string(),
        age: 24,
        is_alive: true,
        gender: Gender::Other("Non-binary".to_string()),
        stuff: vec![String::from("Baguette"), String::from("Croissant")],
    };

    let json_sim = r#"
{
    "name": "Pipouda",
    "age": 27,
    "is_alive": true,
    "gender": "Female",
    "stuff": [],
    "autre-info": "Autre info, mais on s'en fiche un peu"
}
    "#;

    // Serialize the data
    let sim_serialized = serde_json::to_string_pretty(&sim).unwrap();
    println!("Serialized data:\n{}", sim_serialized);

    println!("\n----------------------------\n");

    // Deserialize the data
    let sim_deserialized: Sim = serde_json::from_str(json_sim).unwrap();
    println!("Deserialized data:\n{:#?}", sim_deserialized);
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Sim {
    name: String,
    age: u8, // max age is 255
    is_alive: bool,
    gender: Gender,
    stuff: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum Gender {
    Male,
    Female,
    Other(String),
}
