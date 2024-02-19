use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
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

    HttpResponse::Ok().into()
}

async fn handle_get(location_id: web::Path<String>) -> impl Responder {
    let id = location_id.parse::<usize>();

    if let Ok(id) = id
    {
        let file_name = format!("logs/{}.log", id);
        if let Ok(mut file) = File::open(&file_name) {
            let mut file_content = String::new();
            if let Err(e) = file.read_to_string(&mut file_content) 
            {
                return HttpResponse::InternalServerError().body(format!("Failed to read file: {}", e));
            }
            HttpResponse::Ok().body(file_content)
        } 
        else 
        {
            HttpResponse::NotFound().body(format!("File not found for location {}", location_id))
        }
    }
    else 
    {
        HttpResponse::InternalServerError().into()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/location/{id}", web::post().to(handle_post))
            .route("/location/{id}", web::get().to(handle_get))
    })
    .bind("0.0.0.0:3333")?
    .run()
    .await
}
