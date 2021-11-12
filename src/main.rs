use actix_web::{dev::Service, web::Data, App, HttpServer};

use serverd::{
    controller,
    tools::{log, LogLevel},
};
use tokio_postgres::NoTls;

use std::{
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

fn config() -> (&'static str, u16) {
    let debug = cfg!(debug_assertions);
    let address_port = if debug {
        log(LogLevel::Info("START ACTIX WEB APPLICATION IN DEBUG MOD!"));
        ("127.0.0.1", 5050u16)
    } else {
        ("0.0.0.0", 80u16)
    };
    log(LogLevel::Success(
        std::format!(
            "ACTIX SERVER LISTEN ON {} {}",
            address_port.0,
            address_port.1
        )
        .as_str(),
    ));
    address_port
}

// use MVC model to build
// use easiest to run it at first
#[actix_web::main]
async fn main() -> io::Result<()> {
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
                let address = req.peer_addr().unwrap_or(SocketAddr::V4(SocketAddrV4::new(
                    Ipv4Addr::new(0, 0, 0, 0),
                    0,
                )));
                let ip = address.ip();
                let port = address.port();
                let method = req.method().as_str();
                let info = req.headers().get("User-Agent");
                let socket_version = req.version();
                log(LogLevel::Info(
                    std::format!(
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
                    )
                    .as_str(),
                ));
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
