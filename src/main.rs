#[macro_use]
extern crate actix_web;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use handlebars::Handlebars;
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::time::{Instant};

#[derive(Debug, Clone)]
struct StoredLink {
    value: String,
    created_on: Instant,
}

#[derive(Debug)]
struct AppState {
    links: Vec<StoredLink>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NewLink {
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LinksList {
    links: Vec<String>,
}

fn links(app_state: &Mutex<AppState>) -> Vec<String> {
    let data = app_state.lock().unwrap();
    let mut links = data.links.clone();
    links.sort_by(|l1, l2| l1.created_on.cmp(&l2.created_on));
    links.iter().map(|l| l.value.clone()).collect()
}

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>, app_state: web::Data<Mutex<AppState>>) -> impl Responder {
    let view_model = LinksList { links: links(app_state.get_ref()) };
    let body = hb.render("index", &view_model).unwrap();
    HttpResponse::Ok().body(body)
}

#[post("/links")]
async fn add_link(app_state: web::Data<Mutex<AppState>>, new_link: web::Json<NewLink>) -> impl Responder {
    let mut app_state = app_state.lock().unwrap();
    app_state.links.push(StoredLink { value: new_link.link.clone(), created_on: Instant::now() });
    HttpResponse::Ok()
}

#[get("/links")]
async fn get_links(app_state: web::Data<Mutex<AppState>>) -> impl Responder {
    let links = links(app_state.get_ref());
    HttpResponse::Ok().json(LinksList { links })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", "./static/templates").unwrap();

    let handlebars = web::Data::new(handlebars);
    let app_state = web::Data::new(Mutex::new(AppState { links: Vec::new() }));

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars.clone())
            .app_data(app_state.clone())
            .service(index)
            .service(add_link)
            .service(get_links)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
