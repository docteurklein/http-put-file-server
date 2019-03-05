
use rocket::local::Client;
use rocket::http::Status;
use http_put_file_server::http;

#[test]
fn test_put() {
    let client = Client::new(http::rocket()).unwrap();
    let response = client.put("/test").body("test").dispatch();
    assert_eq!(response.status(), Status::NoContent);
}

#[test]
fn test_get() {
    test_put();
    let client = Client::new(http::rocket()).unwrap();
    let mut response = client.get("/test").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("test".into()));
}

#[test]
fn test_delete() {
    test_put();
    let client = Client::new(http::rocket()).unwrap();
    let response = client.delete("/test").dispatch();
    assert_eq!(response.status(), Status::NoContent);
}
