use std::path::PathBuf;
use actix_files::NamedFile;
use actix_files as fs;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, HttpRequest, Error, web};


#[get("/hola")]
async fn hola() -> impl Responder {
    HttpResponse::Accepted().body("Hola Mundo")
}


async fn index(_req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", ".").show_files_listing())
            .service(hola)
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

