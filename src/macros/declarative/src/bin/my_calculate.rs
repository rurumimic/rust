use std::ops::Add;

struct FirstName {
    value: String,
}

#[allow(dead_code)]
struct LastName {
    value: String,
}

#[allow(dead_code)]
struct Age {
    value: i32,
}

#[derive(Debug)]
struct Pay {
    value: i32,
}

impl FirstName {
    pub fn new(name: &str) -> Result<FirstName, String> {
        if name.len() < 2 {
            Err("Name shoud be at least 2 characters".to_string())
        } else {
            Ok(FirstName {
                value: name.to_string(),
            })
        }
    }

    // pub fn get_value(&self) -> &String {
    //     &self.value
    // }
}

impl LastName {
    pub fn new(name: &str) -> Result<LastName, String> {
        if name.len() < 2 {
            Err("Name shoud be at least 2 characters".to_string())
        } else {
            Ok(LastName {
                value: name.to_string(),
            })
        }
    }
}

impl Age {
    pub fn new(age: i32) -> Result<Age, String> {
        if age < 1 {
            Err("Age shoud be at least 1".to_string())
        } else {
            Ok(Age { value: age })
        }
    }

    // pub fn get_value(&self) -> i32 {
    //     self.value
    // }
}

impl Pay {
    pub fn new(pay: i32) -> Result<Pay, String> {
        if pay < 0 {
            Err("Pay shoud be at least 0".to_string())
        } else {
            Ok(Pay { value: pay })
        }
    }
}

impl Add for Pay {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Pay {
            value: self.value + other.value,
        }
    }
}

macro_rules! generate_get_value {
    ($struct_type:ident) => {
        generate_get_value!($struct_type, String);
    };
    ($struct_type:ident, $return_type:ty) => {
        impl $struct_type {
            pub fn get_value(&self) -> &$return_type {
                &self.value
            }
        }
    };
}

generate_get_value!(FirstName);
// generate_get_value!(LastName);
// generate_get_value!(Age, i32);
generate_get_value!(Pay, i32);

fn calculate_raise(
    first_name: FirstName,
    _last_name: LastName,
    _age: Age,
    current_pay: Pay,
) -> Result<Pay, String> {
    if first_name.get_value() == "Sam" {
        Ok(current_pay + Pay::new(1000)?)
    } else {
        Ok(current_pay)
    }
}

fn main() -> Result<(), String> {
    let current_pay = 1000;
    let raise: Pay = calculate_raise(
        FirstName::new("Sam")?,
        LastName::new("Smith")?,
        Age::new(20)?,
        Pay::new(current_pay)?,
    )?;

    println!("{} -> {}", current_pay, raise.get_value());

    Ok(())
}
