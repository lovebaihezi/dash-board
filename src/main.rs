use actix_web::{dev::Service, web::Data, App, HttpServer};

use serverd::controller;
use std::{
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};
use tokio_postgres::NoTls;
use tracing::info;

fn config() -> (&'static str, u16) {
    let debug = cfg!(debug_assertions);
    let address_port = if debug {
        info!("START ACTIX WEB APPLICATION IN DEBUG MOD!");
        ("127.0.0.1", 5050u16)
    } else {
        ("0.0.0.0", 80u16)
    };
    info!(
        "ACTIX SERVER LISTEN ON {} {}",
        address_port.0, address_port.1
    );
    address_port
}

// use MVC model to build
// use easiest to run it at first
#[actix_web::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();
    let address_port = config();
    let mut config = tokio_postgres::config::Config::new();
    config.host("localhost");
    config.port(5432u16);
    config.user("postgres");
    config.password("postgres");
    config.dbname("postgres");
    let manager = deadpool_postgres::Manager::new(config, NoTls);
    let pool = deadpool_postgres::Pool::builder(manager).build().unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, service| {
                let path = req.path();
                let address = req.peer_addr().unwrap_or_else(|| {
                    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0))
                });
                let ip = address.ip();
                let port = address.port();
                let method = req.method().as_str();
                let info = req.headers().get("User-Agent");
                let socket_version = req.version();
                info!(
                    "{:?} [{:<8} |-> {}] {}:{} [{}]",
                    socket_version,
                    method,
                    path,
                    ip,
                    port,
                    match info {
                        Some(v) => std::format!("{:?}", v),
                        None => "none".to_string(),
                    },
                );
                service.call(req)
            })
            .app_data(Data::new(pool.clone()))
            .configure(controller::os)
            .configure(controller::r#static)
            .configure(controller::verify)
    })
    .bind(address_port)?
    .run()
    .await
}
