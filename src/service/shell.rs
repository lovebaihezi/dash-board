use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws::{CloseCode, CloseReason, Message, ProtocolError, WebsocketContext};
use libc::{c_void, read, write};
use libc_tools::Pty;

use crate::tools::{log, LogLevel};

#[derive(Debug, Message)]
#[rtype(result = "Result<(), ()>")]
pub struct Shell {
    pub pty: Pty,
    // pub actors: Vec<Box<dyn ActorStream<Item = Result<Message, WsProtocolError>, Actor = Shell>>>,
}

impl Drop for Shell {
    fn drop(&mut self) {}
}

impl Actor for Shell {
    type Context = WebsocketContext<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        log(LogLevel::Info("connection start"));
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log(LogLevel::Info("one connection is over"));
    }
}

impl StreamHandler<Result<Message, ProtocolError>> for Shell {
    // shell and context survive when websocket is connected
    fn handle(&mut self, item: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(Message::Text(text)) => match self.pty.pty_fd {
                Some(fd) => unsafe {
                    let mut buf = [0u8; 4096];
                    let s = if text.ends_with('\n') {
                        format!("{}\0", text)
                    } else {
                        format!("{}\n\0", text)
                    };
                    write(fd, s.as_ptr() as *const c_void, s.len());
                    let shell_decoration = read(fd, buf.as_mut_ptr() as *mut c_void, 4096);
                    if shell_decoration > 0 {
                        ctx.binary(buf[..shell_decoration as usize].to_vec());
                    }
                    write(fd, "\n\0".as_ptr() as *const c_void, 2);
                    let size = read(fd, buf.as_mut_ptr() as *mut c_void, 4096);
                    if size > 0 {
                        ctx.binary(buf[..size as usize].to_vec());
                    }
                },
                None => ctx.close(Some(CloseReason {
                    code: CloseCode::Error,
                    description: None,
                })),
            },
            Ok(Message::Ping(msg)) => ctx.pong(&msg),
            Ok(Message::Binary(bin)) => ctx.binary(bin),
            Ok(Message::Close(_)) => ctx.close(None),
            _ => (),
        }
    }
}
