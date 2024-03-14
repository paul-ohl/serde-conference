#![allow(unused)]

use serde::ser::SerializeStruct;
use serde::Serialize;

fn main() {
    let sim = Sim {
        name: "Sim".to_string(),
        age: 24,
        is_alive: true,
    };

    // Serialize the data
    let sim_serialized = serde_json::to_string_pretty(&sim).unwrap();
    println!("Serialized data:\n{}", sim_serialized);
    //
    // // Deserialize the data
    // let sim_deserialized: Sim = serde_json::from_str(&sim_serialized).unwrap();
    // println!("Serialized data:\n{:?}", sim_deserialized);
}

struct Sim {
    name: String,
    age: u8,
    is_alive: bool,
}

impl Serialize for Sim {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Sim", 3)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("age", &self.name)?;
        s.serialize_field("is_alive", &self.name)?;
        s.end()
    }
}
