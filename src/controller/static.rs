use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::tools::get_file_type;

use tokio::fs::read;

use std::io::Result;

pub fn init(ctx: &mut web::ServiceConfig) {
    ctx.service(index).service(file);
}

const PATH: &str = "./dist";

// TODO: get path from env?
// TODO: make a cache?
#[tracing::instrument]
#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(read(format!("{}/{}", PATH, "index.html")).await?))
}

#[tracing::instrument]
#[get("/{path:(assets/).*}")]
async fn file(req: HttpRequest) -> Result<impl Responder> {
    let file_path = req.match_info().query("path").trim_start_matches("..");
    let bytes = read(std::format!("{}/{}", PATH, file_path)).await?;
    let content_type = get_file_type(file_path).unwrap_or("text/plain");
    Ok(HttpResponse::Ok().content_type(content_type).body(bytes))
}
