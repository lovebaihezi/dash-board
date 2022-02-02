use std::error::Error;

use crate::{
    bean::UserLevel,
    tools::crypto::{crypto, crypto_current},
};
use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse,
};
use serde::Deserialize;

pub fn init(ctx: &mut web::ServiceConfig) {
    ctx.service(web::scope("verify"));
}

// #[derive(Debug, Deserialize)]
// struct Bind {
//     level: UserLevel,
// }
// #[tracing::instrument(skip(client))]
// #[post("/qrcode")]
// async fn bing_owner(client: Data<Pool>, data: Json<Bind>) -> Result<HttpResponse, Box<dyn Error>> {
//     let level = data.level.to_string();
//     Ok(if check_bind(&client.get().await?, level.as_str()).await? {
//         // let _token = todo!();
//         HttpResponse::Ok().body("")
//     } else {
//         HttpResponse::Forbidden().body(r#"{"info":"current level has already bind!"}"#)
//     })
// }
//
// #[derive(Debug, Deserialize)]
// struct Confirm {
//     code: u32,
//     level: UserLevel,
// }
// #[tracing::instrument(skip(client))]
// #[post("/check")]
// async fn confirm_code(
//     client: Data<Pool>,
//     data: Json<Confirm>,
// ) -> Result<HttpResponse, Box<dyn Error>> {
//     let key = key_from_user_kind(&client.get().await?, data.level.to_string().as_str()).await?;
//     let code = crypto(crypto_current(), key.as_bytes());
//     Ok(if code == data.code {
//         HttpResponse::Ok().body(r#"{"info":"ok"}"#)
//     } else {
//         HttpResponse::Forbidden().body(r#"{"info":"auth not passed"}"#)
//     })
// }
