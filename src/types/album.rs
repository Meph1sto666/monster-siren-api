use super::song::{self, MSResponse};
use getters2::Getters;
use serde;
use std::{error::Error, result::Result};

#[derive(serde::Deserialize, serde::Serialize, Getters)]
pub struct Album {
    cid: String,
    name: String,
    intro: String,
    belong: String,
    #[serde(alias = "coverUrl")]
    cover_url: String,
    #[serde(alias = "coverDeUrl")]
    cover_de_url: String,
    songs: Vec<song::SongSyn>,
}

#[derive(serde::Deserialize, serde::Serialize, Getters)]
pub struct AlbumSyn {
    cid: String,
    name: String,
    #[serde(alias = "coverUrl")]
    cover_url: String,
    #[serde(alias = "artistes")]
    artists: Vec<String>,
}

/**
 * Fetch the album data from MSR by its ID.
 */
pub async fn fetch_album_details(id: &str) -> Result<Album, Box<dyn Error>> {
    let album: song::MSResponse<Album> = reqwest::get(format!(
        "https://monster-siren.hypergryph.com/api/album/{id}/detail"
    ))
    .await?
    .json::<MSResponse<Album>>()
    .await?;
    Ok(album.data)
}

/**
 * Fetch a list of all albums currently available bt MSR.
 * The list consists of album synopses only
 */
pub async fn fetch_album_list() -> Result<Vec<AlbumSyn>, Box<dyn Error>> {
    let albums: song::MSResponse<Vec<AlbumSyn>> =
        reqwest::get("https://monster-siren.hypergryph.com/api/albums")
            .await?
            .json::<MSResponse<Vec<AlbumSyn>>>()
            .await?;
    Ok(albums.data)
}
