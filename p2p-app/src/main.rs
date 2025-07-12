use std::fs;
use http::{uri};
use rocket::serde::json::Json;
use serde::{/*Deserialize*/ Serialize};
use uuid::Uuid;
use rocket::form::{Form, FromForm};
use rocket::{get, post, launch, routes};
use rocket::fs::{FileServer, TempFile, relative};
use rocket_dyn_templates::{Template, context};
use nanoid::nanoid;


#[derive(FromForm, Debug)]
struct FileUpload<'a> {
    files: Vec<TempFile<'a>>
}


#[derive(Serialize)]
struct UriString {
    message: String,
    uri_string: String
}


#[get("/download-file/<roomid>")]
async fn download_file(roomid: &str) {
    println!("{roomid}");
}


#[post("/file-upload", data = "<files>")]
async fn file_upload(mut files: Form<FileUpload<'_>>) -> std::io::Result<Json<UriString>> {
    let room_id = nanoid!(15);
    let _ = fs::create_dir(format!("tmp/files/{room_id}"));

    for (_i, file) in files.files.iter_mut().enumerate() {
        let uuid = Uuid::new_v4();
        let file_path = format!("tmp/files/{room_id}/{uuid}");
        file.persist_to(&file_path).await?
    }

    let uri_builder = uri::Builder::new()
        .scheme(uri::Scheme::HTTP)
        .authority("127.0.0.1:8000")
        .path_and_query(format!("/download-file/{room_id}"))
        .build()
        .unwrap();

    let uri = UriString {
        message: "Upload succesful".to_string(),
        uri_string: format!("{uri_builder}"),
    };

    Ok(rocket::serde::json::Json(uri))
}


#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, file_upload, download_file])
        .mount("/assets", FileServer::from(relative!("/assets")))
        .attach(Template::fairing())
}
