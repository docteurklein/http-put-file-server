#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use std::io;
use std::path::PathBuf;
use std::fs::{File,create_dir_all,remove_file};

use rocket::Data;
use rocket::http::Status;

#[get("/<path..>")]
fn get(path: PathBuf) -> Option<File> {
    let mut file_path = PathBuf::from("public");
    file_path.push(path.as_path());
    File::open(file_path).ok()
}

#[put("/<path..>", data = "<data>")]
fn put(path: PathBuf, data: Data) -> io::Result<Status> {
    let mut file_path = PathBuf::from("public");
    file_path.push(path.as_path());
    create_dir_all(file_path.parent().unwrap())?;
    data.stream_to_file(file_path.as_path())?;
    Ok(Status::NoContent)
}

#[delete("/<path..>")]
fn delete(path: PathBuf) -> io::Result<Status> {
    let mut file_path = PathBuf::from("public");
    file_path.push(path.as_path());
    remove_file(file_path)?;
    Ok(Status::NoContent)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![get,put,delete])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_put() {
        let client = Client::new(rocket()).unwrap();
        let response = client.put("/test").body("test").dispatch();
        assert_eq!(response.status(), Status::NoContent);
    }

    #[test]
    fn test_get() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/test").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("test".into()));
    }

    #[test]
    fn test_delete() {
        let client = Client::new(rocket()).unwrap();
        let response = client.delete("/test").dispatch();
        assert_eq!(response.status(), Status::NoContent);
    }
}
