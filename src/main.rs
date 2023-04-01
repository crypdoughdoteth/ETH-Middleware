use std::sync::{Mutex, Arc};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use pickledb::{PickleDbDumpPolicy, SerializationMethod, PickleDb};
use serde::{Deserialize, Serialize};
mod types;
mod db;
pub use types::ErrorStates;
use types::State;

//set up routes to access memory
//merkle root
//each leaf is a key in the db


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Suh Dude")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}   

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    println!("🚀 Server started successfully");

    let app_state = web::Data::new(State::init());
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(hello)
            .service(echo)
            //.service(query)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}