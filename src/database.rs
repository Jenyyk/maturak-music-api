use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone, Serialize, Deserialize)]
pub struct MusicRequest {
    pub song_link: String,
    pub id: Uuid,
}

pub struct Database {
    pub data: Vec<MusicRequest>,
}

use once_cell::sync::Lazy;
use std::sync::Mutex;
impl Database {
    pub fn add_song(request: MusicRequest) {
        let mut db = DATABASE.lock().unwrap();
        db.data.push(request);
    }

    pub fn get_songs() -> Vec<MusicRequest> {
        DATABASE.lock().unwrap().data.clone()
    }

    pub fn contains(check_for_link: &str) -> bool {
        let db = DATABASE.lock().unwrap();
        // Fancy one liner to check if current link is in db
        db.data.iter().any(|song| song.song_link == check_for_link)
    }

    pub fn delete_by_id(id: Uuid) -> Result<(), String> {
        let mut db = DATABASE.lock().unwrap();
        let original_length = &db.data.len();
        db.data.retain(|song| song.id != id);
        if original_length != &db.data.len() {
            return Ok(());
        }
        Err(String::from("nothing deleted"))
    }
}

// Global singleton instance
static DATABASE: Lazy<Mutex<Database>> = Lazy::new(|| Mutex::new(Database { data: Vec::new() }));
