use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub enum UserLevel {
    Owner,
    Administrator,
    Common,
}

impl ToString for UserLevel {
    fn to_string(&self) -> String {
        match self {
            UserLevel::Owner => "Owner",
            UserLevel::Administrator => "Administrator",
            UserLevel::Common => "Common",
        }
        .to_string()
    }
}
