use actix_web::{post, App, HttpServer, Responder, HttpRequest};
use std::process::Command;
use std::net::{SocketAddrV4};
use std::env;

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
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("port is required!");
        println!("$ server-kill-switch 8099");
        std::process::exit(1);
    }

    let port = &args[1];

    HttpServer::new(|| App::new().service(kill))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}
