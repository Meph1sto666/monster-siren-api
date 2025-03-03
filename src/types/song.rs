use getters2::Getters;
use reqwest;
use serde;
use std::{error::Error, result::Result};

#[derive(serde::Deserialize, serde::Serialize, Getters)]
pub struct SongSyn {
    cid: String,
    name: String,
    #[serde(alias = "albumCid")]
    album_cid: Option<String>,
    #[serde(alias = "artistes")]
    artists: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Getters)]
pub struct Song {
    cid: String,
    name: String,
    #[serde(alias = "albumCid")]
    album_cid: String,
    #[serde(alias = "sourceUrl")]
    source_url: String,
    #[serde(alias = "lyricUrl")]
    lyrics_url: Option<String>,
    #[serde(alias = "mvUrl")]
    mv_url: Option<String>,
    #[serde(alias = "mvCoverUrl")]
    mv_cover_url: Option<String>,
    artists: Vec<String>,
}

impl PartialEq for Song {
    fn eq(&self, other: &Self) -> bool {
        self.cid == other.cid
            && self.name == other.name
            && self.album_cid == other.album_cid
            && self.source_url == other.source_url
            && self.lyrics_url == other.lyrics_url
            && self.mv_url == other.mv_url
            && self.mv_cover_url == other.mv_cover_url
            && self.artists == other.artists
    }
}

/**
 * Fetch the list of songs currently available on MSR.
 * The list consists of song synapsis's only
 */
pub async fn fetch_song_list() -> Result<Vec<SongSyn>, Box<dyn Error>> {
    Ok(fetch_songs_raw().await?.data.list)
}

/**
 * Fetch the current auto-play song (set by MSR) of songs currently available on MSR.
 * The song is only the song ID
 */
pub async fn fetch_autoplay_song_id() -> Result<String, Box<dyn Error>> {
    Ok(fetch_songs_raw().await?.data.autoplay)
}

/**
 * Fetch the details of a song by its ID
 */
pub async fn fetch_song_details(id: &str) -> Result<Song, Box<dyn Error>> {
    let res: MSResponse<Song> = reqwest::get(format!(
        "https://monster-siren.hypergryph.com/api/song/{id}"
    ))
    .await?
    .json::<MSResponse<Song>>()
    .await?;
    Ok(res.data)
}

#[derive(serde::Deserialize)]
struct ResponseData {
    list: Vec<SongSyn>,
    autoplay: String,
}
#[derive(serde::Deserialize)]
#[allow(unused)]
pub(crate) struct MSResponse<T> {
    code: i32,
    msg: String,
    pub(crate) data: T,
}
async fn fetch_songs_raw() -> Result<MSResponse<ResponseData>, Box<dyn Error>> {
    let res: MSResponse<ResponseData> =
        reqwest::get("https://monster-siren.hypergryph.com/api/songs")
            .await?
            .json::<MSResponse<ResponseData>>()
            .await?;
    Ok(res)
}
