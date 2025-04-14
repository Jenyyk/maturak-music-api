mod database;
use crate::database::Database;
use crate::database::MusicRequest;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, web};

// API get request to list current queue
#[get("/music")]
async fn get_music() -> impl Responder {
    let songs = Database::get_songs();
    HttpResponse::Ok().json(songs)
}

// API post request to add a song to queue
use uuid::Uuid;
#[post("/music")]
async fn add_song(body: web::Json<String>) -> impl Responder {
    let data: String = body.into_inner();
    if Database::contains(&data) {
        return HttpResponse::Conflict().body("Song link already found");
    }
    Database::add_song(MusicRequest {
        song_link: data,
        id: Uuid::new_v4(),
    });
    HttpResponse::Created().body("Succesfully added your song")
}
// API delete request
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct DeleteRequest {
    pub id: String,
    pub password: String,
}
use dotenv::dotenv;
#[delete("/music")]
async fn delete_song(body: web::Json<DeleteRequest>) -> impl Responder {
    let data: DeleteRequest = body.into_inner();
    dotenv().ok();
    if data.password != std::env::var("DATABASE_PASSWORD").unwrap_or(String::from("admin")) {
        return HttpResponse::Unauthorized().body("Invalid password");
    }
    let id: Uuid = match Uuid::parse_str(&data.id) {
        Ok(uuid) => uuid,
        _ => return HttpResponse::BadRequest().body("Invalid UUID for song"),
    };

    if Database::delete_by_id(id).is_ok() {
        return HttpResponse::Ok().body("Deleted succesfully");
    }
    HttpResponse::NotFound().body("Song not found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("https://maturak26ab.cz")
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec!["Content-Type"]);

        App::new()
            .service(get_music)
            .service(add_song)
            .service(delete_song)
            .wrap(cors)
    })
    .bind(("127.0.0.1", 3030))
    .unwrap()
    .run()
    .await
}
