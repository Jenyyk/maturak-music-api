// cant be arsed, serde says so
#![allow(dead_code)]

use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SpotifyTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Deserialize, Debug)]
pub struct TrackInfo {
    pub name: String,
    pub album: Album,
}

#[derive(Deserialize, Debug)]
pub struct Album {
    pub name: String,
    pub images: Vec<Image>,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

// Function to retrieve the Spotify access token using Client Credentials flow
async fn get_spotify_token(
    client: &Client,
    client_id: &str,
    client_secret: &str,
) -> Result<String> {
    let credentials = format!("{}:{}", client_id, client_secret);
    let encoded = general_purpose::STANDARD.encode(credentials);

    let params = [("grant_type", "client_credentials")];

    let resp = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", format!("Basic {}", encoded))
        .form(&params)
        .send()
        .await
        .context("Failed to request token")?
        .json::<SpotifyTokenResponse>()
        .await
        .context("Failed to parse token response")?;

    Ok(resp.access_token)
}

// Function to fetch song details using only the song ID
pub async fn get_song_details(
    client: &Client,
    song_id: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<(String, String)> {
    let token = get_spotify_token(client, client_id, client_secret).await?;

    let url = format!("https://api.spotify.com/v1/tracks/{}", song_id);

    let resp = client
        .get(&url)
        .bearer_auth(&token)
        .send()
        .await
        .context("Failed to fetch track data")?
        .json::<TrackInfo>()
        .await
        .context("Failed to parse track response")?;

    // Step 4: Extract and return the song name and image URL
    let song_name = resp.name;
    let song_image_url = resp
        .album
        .images
        .first()
        .map(|img| img.url.clone())
        .unwrap_or_else(|| String::from("No image available"));

    Ok((song_name, song_image_url))
}
