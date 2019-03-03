#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use std::io;
use std::path::PathBuf;
use std::fs::{File,create_dir_all};

use rocket::Data;
use rocket::http::Status;

#[get("/<path..>")]
fn get(path: PathBuf) -> Option<File> {
    let filename = format!("public/{path}", path = path.as_path().display());
    File::open(&filename).ok()
}

#[put("/<path..>", data = "<data>")]
fn put(path: PathBuf, data: Data) -> io::Result<Status> {
    let mut file_path = PathBuf::from("public");
    file_path.push(path.as_path());
    create_dir_all(file_path.parent().unwrap())?;
    data.stream_to_file(file_path.as_path())?;
    Ok(Status::NoContent)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![put,get])
        .launch();
}
