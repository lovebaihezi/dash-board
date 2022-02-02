use actix_multipart::Multipart;
use actix_web::{post, web, Error, Responder};
use futures_util::StreamExt;

pub fn init(ctx: &mut web::ServiceConfig) {
    ctx.service(web::scope("/file"));
}
