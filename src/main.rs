use actix_web::{get, post, web, App, HttpServer, Responder, HttpRequest};
use std::process::Command;
use std::net::{Ipv4Addr, IpAddr, SocketAddrV4};
use std::str::FromStr;

#[post("/kill")]
async fn kill(req: HttpRequest) -> impl Responder {
    let conn_info = req.connection_info();
    let addr_str = conn_info.realip_remote_addr().expect("failed to get ip");
    let socket_addr: SocketAddrV4 = addr_str.parse().expect("failed to parse addr_str");

    println!("{}", socket_addr.ip());
    if !socket_addr.ip().is_private() {
        return "failed to shutdown";
    }

    Command::new("sh")
        .arg("-c")
        .arg("shutdown now")
        .output()
        .expect("failed to execute process");

    "success"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(kill))
        .bind("127.0.0.1:8090")?
        .run()
        .await
}
