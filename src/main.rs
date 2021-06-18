#[macro_use]
extern crate actix_web;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use storage::Storage;

mod storage;

#[derive(Debug, Serialize, Deserialize)]
struct NewLink {
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LinksList {
    links: Vec<String>,
}

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>, storage: web::Data<Storage>) -> impl Responder {
    let links = storage.links();
    let view_model = LinksList { links };
    let body = hb.render("index", &view_model).unwrap();
    HttpResponse::Ok().body(body)
}

#[post("/links")]
async fn add_link(storage: web::Data<Storage>, new_link: web::Json<NewLink>) -> impl Responder {
    storage.add_link(new_link.into_inner().link);
    HttpResponse::Ok()
}

#[get("/links")]
async fn get_links(storage: web::Data<Storage>) -> impl Responder {
    let links = storage.links();
    HttpResponse::Ok().json(LinksList { links })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", "./static/templates").unwrap();

    let handlebars = web::Data::new(handlebars);
    let storage = web::Data::new(Storage::new());

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars.clone())
            .app_data(storage.clone())
            .service(index)
            .service(add_link)
            .service(get_links)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
