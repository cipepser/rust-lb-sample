#[macro_use]
extern crate lazy_static;

use actix_web::client::Client;
use actix_web::{
    middleware, web, App, HttpServer,
};

use std::sync::atomic::AtomicUsize;
use std::sync::Mutex;
use url::Url;
use failure::_core::panicking::panic_fmt;

mod req;

lazy_static! {
    static ref CURRENT_INDEX: AtomicUsize = AtomicUsize::new(0);
    static ref SERVERS: Mutex<Vec<Server>> = {
        let base_info = vec![
            Server {
                url: req::create_base_url("127.0.0.1", 8000).unwrap(),
                is_alive: true,
            },
            Server {
                url: req::create_base_url("127.0.0.1", 8001).unwrap(),
                is_alive: true,
            },
            Server {
                url: req::create_base_url("127.0.0.1", 8002).unwrap(),
                is_alive: true,
            },
        ];
        Mutex::new(base_info)
    };
}

pub struct Server {
    pub url: Url,
    pub is_alive: bool,
}

impl Server {
    pub fn new(host: String, port: u16) -> Server {
        Server {
            url: req::create_base_url(&host, port).unwrap(),
            is_alive: true,
        }
    }
}

// TODO: unimplemented
//pub async fn forward() -> {}

// TODO: unimplemented
//pub async fn active_check() -> {}

// TODO: unimplemented
pub fn passive_check() {
    panic!("unimplemented");
}

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    passive_check();
    print!("run proxy");
    HttpServer::new(move || {
        App::new()
            .data(Client::new())
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(forward))
    })
        .bind(("127.0.0.1", 3000))?
        .system_exit()
        .start()
        .await
}