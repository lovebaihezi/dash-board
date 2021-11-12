use actix_multipart::Multipart;
use actix_web::{post, web, Error, Responder};
use deadpool_postgres::Pool;
use futures_util::StreamExt;

pub fn init(ctx: &mut web::ServiceConfig) {
    ctx.service(web::scope("/file").service(upload_file));
}

#[post("/upload")]
async fn upload_file(
    mut payload: Multipart,
    _pool: web::Data<Pool>,
) -> Result<impl Responder, Error> {
    while let Some(v) = payload.next().await {
        let mut item = v?;
        println!("{:?}", &item);
        while let Some(c) = item.next().await {
            println!("{:?}", std::str::from_utf8(&c?));
        }
    }
    Ok("")
}
