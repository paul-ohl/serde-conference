#![allow(unused)]
use serde::ser::SerializeStruct;

fn main() {
    let sim = Sim {
        name: "Poupidou".to_string(),
        age: 24,
        is_alive: true,
        gender: Gender::Other("Non-binary".to_string()),
        stuff: vec![String::from("Baguette"), String::from("Croissant")],
    };

    // Serialize the data
    let sim_serialized = serde_json::to_string_pretty(&sim).unwrap();
    println!("Serialized data:\n{}", sim_serialized);
}

struct Sim {
    name: String,
    age: u8, // max age is 255
    is_alive: bool,
    gender: Gender,
    stuff: Vec<String>,
}

impl serde::Serialize for Sim {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Sim", 5)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("age", &self.age)?;
        state.serialize_field("is_alive", &self.is_alive)?;
        state.serialize_field("gender", &self.gender)?;
        state.serialize_field("stuff", &self.stuff)?;
        state.end()
    }
}

enum Gender {
    Male,
    Female,
    Other(String),
}

impl serde::Serialize for Gender {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Gender::Male => serializer.serialize_unit_variant("Gender", 0, "Male"),
            Gender::Female => serializer.serialize_unit_variant("Gender", 1, "Female"),
            Gender::Other(ref value) => {
                serializer.serialize_newtype_variant("Gender", 2, "Other", value)
            }
        }
    }
}
