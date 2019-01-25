mod routes;

use std::{ env };
use models;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let addr_server = "127.0.0.1:8080";
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let sys = actix::System::new("Plums");
    let _ = actix_web::server::new(|| routes::create_new_router())
        .bind(addr_server.to_owned())
        .expect("Can not bind to 127.0.0.1:8080")
        .start();
    println!("Starting http server: 127.0.0.1:8080");
    let _ = sys.run();
}