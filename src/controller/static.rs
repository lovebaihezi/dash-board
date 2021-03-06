use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::tools::file_content_type;

pub fn init(ctx: &mut web::ServiceConfig) {
    ctx.service(index).service(file);
}

static PATH: &str = "./dist";

// TODO: get path from env?
// TODO: make a cache?
#[get("/")]
async fn index() -> std::io::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(tokio::fs::read(format!("{}/{}", PATH, "index.html")).await?))
}

#[get("/{path:(assets/).*}")]
async fn file(req: HttpRequest) -> std::io::Result<impl Responder> {
    let file_path = req.match_info().query("path").trim_start_matches("..");
    let bytes = tokio::fs::read(std::format!("{}/{}", PATH, file_path)).await?;
    let content_type = file_content_type(file_path).unwrap_or("text/plain");
    Ok(HttpResponse::Ok().content_type(content_type).body(bytes))
}
