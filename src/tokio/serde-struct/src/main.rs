use serde::{de, Deserialize, Deserializer, Serialize};

use std::{fmt, str::FromStr};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    name: Option<String>,
    age: Option<u8>,
    phones: Vec<String>,
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

fn main() {
    let person = Person {
        name: Some("".to_string()),
        age: Some(28),
        phones: vec!["+44 1234567".to_string(), "+44 2345678".to_string()],
    };

    let serialized = serde_json::to_string(&person).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Person = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
