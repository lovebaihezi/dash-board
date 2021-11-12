use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BitType {
    B(u64),
    KB(u64),
    MB(u64),
    GB(u64),
    TB(u64),
    PB(u64),
    YB(u64),
    Unknown(u64),
}

impl BitType {
    pub fn new(s: &str) -> Option<BitType> {
        let mut name_value = s.split(" ");
        let size = match name_value.next().unwrap().parse::<u64>() {
            Ok(v) => v,
            Err(_) => panic!("value is out of unsigned long!"),
        };
        Some(match name_value.next()?.to_ascii_uppercase().as_str() {
            "B" => BitType::B(size),
            "KB" => BitType::KB(size),
            "MB" => BitType::MB(size),
            "GB" => BitType::GB(size),
            "TB" => BitType::TB(size),
            "PB" => BitType::PB(size),
            "YB" => BitType::YB(size),
            _ => BitType::Unknown(size),
        })
    }
}
