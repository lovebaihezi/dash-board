use crate::{
    service::Shell,
    tools::{cpu_info::cpu_info, cpu_stat::cpu_stat, log, mem_info::mem_info, LogLevel},
};
use actix_web::{
    get, post,
    web::{self, Payload, Query},
    Error, HttpRequest, HttpResponse, Responder,
};
use actix_web_actors::ws;
use libc::{termios, winsize, NCCS};
use libc_tools::{Pty, PtyError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io;
pub fn init(ctx: &mut web::ServiceConfig) {
    ctx.service(web::scope("/os").service(proc).service(shell));
}

#[derive(Debug, Deserialize, Serialize)]
enum OsInfoType {
    MemInfo,
    CpuInfo,
    CpuStat,
    PidInfo,
}

#[derive(Deserialize, Serialize)]
struct OsInfoQuery {
    path: OsInfoType,
}

// will accept path like {baseurl}/os/proc?info={...OsInfoType}
#[post("/proc")]
async fn proc(Query(OsInfoQuery { path }): Query<OsInfoQuery>) -> io::Result<impl Responder> {
    let res = HttpResponse::Ok()
        .content_type("application/json")
        .body(match path {
            OsInfoType::CpuInfo => json!(cpu_info()?).to_string(),
            OsInfoType::CpuStat => json!(cpu_stat()?).to_string(),
            OsInfoType::MemInfo => json!(mem_info()?).to_string(),
            OsInfoType::PidInfo => json!({}).to_string(),
        });
    Ok(res)
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    size: String,
}

unsafe trait ToS {
    fn to_string(&self) -> String;
}

unsafe impl ToS for PtyError {
    fn to_string(&self) -> String {
        match self {
            PtyError::CreatePtyFailed(c) => std::format!("create pty failed! {}", c),
            PtyError::ForkFailed(c) => std::format!("pty fork failed!{}", c),
        }
    }
}

#[get("/shell")]
async fn shell(req: HttpRequest, stream: Payload) -> Result<HttpResponse, Error> {
    let mut termios = termios {
        c_iflag: 0u32,
        c_oflag: 0u32,
        c_cflag: 0u32,
        c_lflag: 0u32,
        c_line: 0u8,
        c_cc: [0u8; NCCS],
        c_ispeed: 0u32,
        c_ospeed: 0u32,
    };
    let mut winsize = winsize {
        ws_row: 10u16,
        ws_col: 10u16,
        ws_xpixel: 10u16,
        ws_ypixel: 10u16,
    };
    match Pty::new(&mut termios as *mut termios, &mut winsize as *mut winsize) {
        Ok(pty) => {
            let shell = ws::start(Shell { pty }, &req, stream);
            shell
        }
        Err(v) => {
            log(LogLevel::Error(v.to_string().as_str()));
            Ok(HttpResponse::InternalServerError()
                .body(r#"{"info":"create pty terminal failed!}""#))
        }
    }
}
