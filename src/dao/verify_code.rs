use std::{error::Error, fmt::Display};

use deadpool_postgres::Client;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
enum QueryError<'a> {
    NotFound(&'a str),
}

impl Error for QueryError<'_> {}

impl Display for QueryError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(v) => f.write_str(v),
        }
    }
}

#[inline]
#[allow(dead_code)]
pub async fn key_from_token(client: &Client, token: &str) -> Result<String, Box<dyn Error>> {
    let rows = client
        .query(
            "SELECT DISTINCT key FROM dash WHERE $1::TEXT = token",
            &[&token],
        )
        .await?;
    match rows.first() {
        Some(v) => Ok(v.try_get(0)?),
        None => Err(Box::new(QueryError::NotFound(
            r#"{"info":"token not found"}"#,
        ))),
    }
}

#[inline]
#[allow(dead_code)]
pub async fn key_from_phone(client: &Client, phone: &str) -> Result<String, Box<dyn Error>> {
    let rows = client
        .query(
            "SELECT DISTINCT key FROM dash WHERE $1::TEXT = token",
            &[&phone],
        )
        .await?;
    match rows.first() {
        Some(v) => Ok(v.try_get(0)?),
        None => Err(Box::new(QueryError::NotFound(
            r#"{"info":"phone not found"}"#,
        ))),
    }
}

#[inline]
pub async fn key_from_user_kind(client: &Client, kind: &str) -> Result<String, Box<dyn Error>> {
    let rows = client
        .query(
            "SELECT DISTINCT key FROM dash WHERE $1::TEXT = kind",
            &[&kind],
        )
        .await?;
    match rows.first() {
        Some(v) => Ok(v.try_get(0)?),
        None => Err(Box::new(QueryError::NotFound(
            r#"{"info":"kind not valid!"}"#,
        ))),
    }
}

#[inline]
pub async fn check_bind(client: &Client, level: &str) -> Result<bool, Box<dyn Error>> {
    let rows = client
        .query(
            "SELECT DISTINCT bind FROM dash WHERE $1::TEXT = levels",
            &[&level],
        )
        .await?;
    match rows.first() {
        Some(v) => {
            let v: bool = dbg!(v.try_get(0)?);
            Ok(dbg!(v))
        }
        None => Err(Box::new(QueryError::NotFound(
            r#"{"info":"level not found"}""#,
        ))),
    }
}

#[inline]
pub async fn update_to_bind(client: &Client) -> Result<(), Box<dyn Error>> {
    client.query("UPDATE dash SET bind = True", &[]).await?;
    todo!();
    // Ok(())
}
