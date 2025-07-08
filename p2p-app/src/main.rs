// #[macro_use] extern crate rocket;
use rocket::form::{Form, FromForm};
use rocket::{get, post, launch, routes};
use rocket::fs::{FileServer, TempFile, relative};
use rocket_dyn_templates::{Template, context};

#[derive(FromForm, Debug)]
struct FileUpload<'a> {
    files: Vec<TempFile<'a>>
}

#[post("/file-upload", data = "<files>")]
fn file_upload(files: Form<FileUpload<'_>>) -> std::io::Result<()> {
    println!("{:?}", files);
    // let files = files.into_inner();
    // for (i, file) in files.iter().enumerate() {
    //     println!("{i}: {}", file.len());
    // }
    Ok(())
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, file_upload])
        .mount("/assets", FileServer::from(relative!("/assets")))
        .attach(Template::fairing())
}
