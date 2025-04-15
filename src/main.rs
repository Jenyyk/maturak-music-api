mod database;
mod spotifyapi;

use crate::database::Database;
use crate::database::MusicRequest;
use dotenv::dotenv;

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
    let link_trimmed = data.clone().split("?").next().unwrap().to_string();
    if Database::contains(&link_trimmed) {
        return HttpResponse::Conflict().body("Song link already found");
    }
    let spotify_id = link_trimmed.split("/").last().unwrap().to_string();

    // fetch full song details
    let client = reqwest::Client::new();
    dotenv().ok();
    let client_id = std::env::var("SPOTIFY_CLIENT_ID").expect("No spotify client id supplied");
    let client_secret =
        std::env::var("SPOTIFY_CLIENT_SECRET").expect("No spotify client secret supplied");
    let (song_name, image_link) =
        spotifyapi::get_song_details(&client, &spotify_id, &client_id, &client_secret)
            .await
            .unwrap_or((String::from("Not found"), String::from("Not found")));
    Database::add_song(MusicRequest {
        song_link: link_trimmed,
        uuid: Uuid::new_v4(),
        spotify_id,
        name: song_name,
        image_link,
        votes: 0,
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

// API voting
async fn upvote_song(body: web::Path<String>) -> impl Responder {
    let data = body.into_inner();
    let id: Uuid = match Uuid::parse_str(&data) {
        Ok(uuid) => uuid,
        _ => return HttpResponse::BadRequest().body("Invalid UUID for song"),
    };

    if Database::upvote_by_id(id).is_ok() {
        return HttpResponse::Ok().body("Upvoted successfully");
    }
    HttpResponse::NotFound().body("Song not found")
}
async fn downvote_song(body: web::Path<String>) -> impl Responder {
    let data = body.into_inner();
    let id: Uuid = match Uuid::parse_str(&data) {
        Ok(uuid) => uuid,
        _ => return HttpResponse::BadRequest().body("Invalid UUID for song"),
    };

    if Database::downvote_by_id(id).is_ok() {
        return HttpResponse::Ok().body("Upvoted successfully");
    }
    HttpResponse::NotFound().body("Song not found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("https://maturak26ab.cz")
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
            .allowed_headers(vec!["Content-Type"]);

        App::new()
            .service(get_music)
            .service(add_song)
            .service(delete_song)
            .service(web::resource("/music/upvote/{id}").route(web::patch().to(upvote_song)))
            .service(web::resource("/music/downvote/{id}").route(web::patch().to(downvote_song)))
            .wrap(cors)
    })
    .bind(("0.0.0.0", 3030))
    .unwrap()
    .run()
    .await
}
