#[macro_use]
extern crate lazy_static;

use std::sync::atomic::AtomicUsize;
use std::sync::Mutex;
use url::Url;

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

fn main() {
    print!("hello");
}