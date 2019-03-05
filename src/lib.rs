#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod http {
    use std::io;
    use std::env;
    use std::path::PathBuf;
    use std::fs::{File,create_dir_all,remove_file};

    use rocket::Data;
    use rocket::http::Status;

    #[get("/<path..>")]
    fn get(path: PathBuf) -> Option<File> {
        File::open(file_path(path)).ok()
    }

    #[put("/<path..>", data = "<data>")]
    fn put(path: PathBuf, data: Data) -> io::Result<Status> {
        let file_path = file_path(path);
        create_dir_all(file_path.parent().unwrap())?;
        data.stream_to_file(file_path.as_path())?;
        Ok(Status::NoContent)
    }

    #[delete("/<path..>")]
    fn delete(path: PathBuf) -> io::Result<Status> {
        remove_file(file_path(path))?;
        Ok(Status::NoContent)
    }

    pub fn rocket() -> rocket::Rocket {
        rocket::ignite().mount("/", routes![get,put,delete])
    }

    fn file_path(path: PathBuf) -> PathBuf {
        let public_dir = env::var("PUBLIC_DIR");
        let public_dir = public_dir
            .as_ref()
            .map(String::as_str)
            .unwrap_or("public");

        let mut file_path = PathBuf::from(public_dir);
        file_path.push(path);
        file_path
    }
}
