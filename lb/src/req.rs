extern crate url;

use std::fmt;
use std::fmt::Display;
use std::io::Error as IOError;
use url::Url;
use url::ParseError as UrlParseError;
use failure::{Backtrace, Context, Fail};
use std::net::ToSocketAddrs;
use actix_web::client::{Client, ClientRequest};
use actix_web::{dev, http::Uri, web}
use crate::req::ReqErrorKind::NotFoundSockAddr;

#[derive(Debug, Fail)]
pub enum ReqErrorKind {
    #[fail(display = "IO error")]
    Io,
    #[fail(display = "Cannot parse url")]
    UrlParse,
    #[fail(display = "SockAddr not found")]
    NotFoundSockAddr,
}

#[derive(Debug)]
pub struct Error {
    inner: Context<ReqErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn new(inner: Context<ReqErrorKind>) -> Error {
        Error { inner }
    }

    pub fn kind(&self) -> &ReqErrorKind {
        self.inner.get_context()
    }
}

impl From<ReqErrorKind> for Error {
    fn from(kind: ReqErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ReqErrorKind>> for Error {
    fn from(inner: Context<ReqErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Error {
        Error {
            inner: error.context(ReqErrorKind::Io)
        }
    }
}

impl From<UrlParseError> for Error {
    fn from(error: UrlParseError) -> Error {
        Error {
            inner: error.context(ReqErrorKind::UrlParse)
        }
    }
}

pub fn create_base_url(host: &str, port: u16) -> Result<Url, Error> {
    let url = Url::parse(&format!(
        "http://{}",
        (host, port).to_socket_addrs()?
            .next().ok_or(NotFoundSockAddr)?
    ))?;
    Ok(url)
}

#[test]
fn test_create_base_url() {
    let actual = create_base_url("127.0.0.1", 8000);
    assert!(actual.is_ok());
    let actual = actual.unwrap();

    assert_eq!(actual.as_str(), "http://127.0.0.1:8000/");
}

pub fn create_forwarded_req(
    client: &web::Data<Client>,
    head: &dev::RequestHead,
    new_url: &str
) -> ClientRequest {
    let forwarded_req = client.request_from(new_url, head).no_decompress();
    if let Some(addr) = head.peer_addr {
        forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
    } else {
        forwarded_req
    }
}