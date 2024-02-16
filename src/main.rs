use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs::OpenOptions;
use std::io::Write;
use chrono::{Local, DateTime};

async fn handle_post(location_id: web::Path<String>, payload: String) -> impl Responder {
    println!("{},{}", location_id, payload);
    
    // Determine file name from location ID
    let file_name = format!("logs/{}.log", location_id);

    // Open the file with append mode, create it if it doesn't exist
    let mut file = match OpenOptions::new().append(true).create(true).open(&file_name) {
        Ok(file) => file,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to open file: {}", e)),
    };

    // Write the payload to the file
    let now: DateTime<Local> = Local::now();
    if let Err(e) = writeln!(file, "{}: {}",now, payload) {
        return HttpResponse::InternalServerError().body(format!("Failed to write to file: {}", e));
    }

    HttpResponse::Ok().body("Request logged successfully")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/location/{id}", web::post().to(handle_post))
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await
}
