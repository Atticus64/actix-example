use std::path::PathBuf;
use actix_files::NamedFile;
use actix_files as fs;
use actix_web::{get,Result, App, HttpResponse, HttpServer, Responder, HttpRequest, Error, web};

pub mod structs;
use structs::{Person, Alumno, Data};


#[get("/hola")]
async fn hola() -> impl Responder {
    HttpResponse::Accepted().body("Hola Mundo")
}

#[get("/user/{name}")]
async fn json(name:  web::Path<String> ) -> Result<impl Responder> {
    let dev = Person {
        name: name.to_string()
    };
    Ok(web::Json(dev))
}

#[get("/student")]
async fn student_data() -> Result<impl Responder> {
    let juan = Alumno {
        name: "Juan Gallego".to_string(),
        aprobado: true,
        escuela: "Federal #2".to_string(),
        salon: 502
    };
    let data = Data {
        tipo: "Datos de alumno".to_string(),
        student: juan,
        notes: [9,8,8,9,8,7,10,8,9,10]
    };
    Ok(web::Json(data))
}

#[get("/data")]
async fn datos() -> Result<impl Responder> {
    let jorge = Alumno {
        name: String::from("Jorge Calles"),
        aprobado: false,
        escuela: String::from("Harvard"),
        salon: 122
    };
    let pepe = Alumno {
        name: String::from("Pepe Macias"),
        aprobado: true,
        escuela: String::from("Harvard"),
        salon: 305
    };
    let maria = Alumno {
        name: String::from("Maria Orduñez"),
        aprobado: true,
        escuela: String::from("Harvard"),
        salon: 507
    };
    let pedro = Alumno {
        name: String::from("Pedro Sillas"),
        aprobado: true,
        escuela: String::from("Harvard"),
        salon: 243
    };


    let alumnos: [Alumno;4] = [jorge, pepe, maria, pedro];
    
    Ok(web::Json(alumnos))
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
            .service(json)
            .service(student_data)
            .service(datos)
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

