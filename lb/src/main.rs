#[macro_use]
extern crate lazy_static;

use actix_web::client::{Client, ClientResponse, SendRequestError};
use actix_web::{
    dev::{Decompress, Payload, PayloadStream, RequestHead},
    middleware, web, App, HttpServer,
    HttpRequest, HttpResponse, Error,
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

fn get_new_url() -> String {
    // TODO: unimplemented
    panic!("unimplemented")
}

pub async fn forward(
    req: HttpRequest,
    body: web::Bytes,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let head = req.head();
    let mut res;
    loop {
        if let Ok(raw_res) = active_check(
            &client, head, &body, get_new_url().as_str(),
        ).await {
            res = raw_res;
            break;
        }
    }

    let mut client_resp = HttpResponse::build(res.status());

    for (header_name, header_value)
        in res.headers()
        .iter()
        .filter(|(h, _)| *h != "connection") {
        client_resp.header(header_name.clone(), header_value.clone())
    }
    Ok(client_resp.body(res.body().await?))
}

pub async fn active_check(
    client: &web::Data<Client>,
    head: &RequestHead,
    body: &web::Bytes,
    new_url: &str,
) -> Result<ClientResponse<Decompress<Payload<PayloadStream>>>, SendRequestError> {
    // TODO: unimplemented
    panic!("unimplemented");
}

pub fn passive_check() {
    // TODO: unimplemented
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