mod bean;
pub mod controller;
mod dao;
mod middleware;
pub mod model;
mod service;
pub mod tools;

pub const DEBUG: bool = cfg!(debug_assertions);
pub type PgPool = sqlx::Pool<sqlx::Postgres>;
pub const PGSQLURL: &str = if DEBUG {
    "postgres://postgres:postgres@localhost/test"
} else {
    match option_env!("PGSQLURL") {
        Some(v) => v,
        None => "",
    }
};
