use serde::Serialize;

#[derive(Serialize)]
pub struct Person {
    pub name: String,
}

#[derive(Serialize)]
pub struct Alumno {
    pub name: String,
    pub aprobado: bool,
    pub escuela: String, 
    pub salon: u32
}

#[derive(Serialize)]
pub struct Data {
    pub tipo: String,
    pub student: Alumno,
    pub notes: [i32;10] 
}