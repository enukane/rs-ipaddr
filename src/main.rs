#[macro_use] extern crate rocket;

use std::net::IpAddr;
use rocket::http::Status;
use rocket::request::{Request, Outcome, FromRequest};

struct ClientAddr {ip: IpAddr}

#[derive(Debug)]
enum ClientAddrError {
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientAddr {
    type Error = ClientAddrError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.real_ip() {
            Some(addr_str) => Outcome::Success(ClientAddr{ip: addr_str }),
            None => match req.client_ip()  {
                None => Outcome::Failure((Status::InternalServerError, ClientAddrError::Invalid)),
                Some(addr) => Outcome::Success(ClientAddr{ip: addr}),
            }
        }
    }
}

#[get("/")]
fn index(addr: ClientAddr) -> String {
    let addr_str = addr.ip.to_string();
    return addr_str
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}