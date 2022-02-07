use actix_web::{dev::Service, App, HttpServer};

use dashboard::controller;

use tracing_appender::{non_blocking, rolling};

pub(crate) const DEBUG: bool = cfg!(debug_assertions);

fn config() -> (&'static str, u16) {
    let address_port = if DEBUG {
        tracing::debug!("START ACTIX WEB APPLICATION IN DEBUG MOD!");
        ("127.0.0.1", 5050u16)
    } else {
        ("0.0.0.0", 80u16)
    };
    tracing::trace!(
        "ACTIX SERVER LISTEN ON {}:{}",
        address_port.0,
        address_port.1
    );
    address_port
}

// use MVC model to build
// use easiest to run it at first
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (no_block, _guard) = non_blocking(rolling::daily("/tmp", "dashboard.log"));
    if DEBUG {
        tracing_subscriber::fmt().with_writer(no_block).init();
    } else {
        tracing_subscriber::fmt().init();
    }
    let address_port = config();
    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, service| {
                let path = req.path();
                let address = req.peer_addr();
                let ip = address.map(|v| v.ip());
                let port = address.map(|v| v.port());
                let method = req.method().as_str();
                let user_agent = req
                    .headers()
                    .get("User-Agent")
                    .map(|v| v.to_str().unwrap_or(""))
                    .unwrap_or("");
                let socket_version = req.version();
                tracing::info!(
                    "<{:?}> [{:<8}] {{{}}} |{:?}:{:?}| ({})",
                    socket_version,
                    method,
                    path,
                    ip,
                    port,
                    user_agent
                );
                service.call(req)
            })
            .configure(controller::os)
            .configure(controller::r#static)
            .configure(controller::verify)
    })
    .bind(address_port)?
    .run()
    .await
}
