use super::album::{fetch_album_details, Album};
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

impl Song {
    /**
     * Get the release date of a song from the source URL.
     */
    pub fn get_release_date(&self) -> chrono::NaiveDate {
        let suff: &str = self.source_url.split("siren/audio/").collect::<Vec<&str>>()[1];
        let date_str: &str = suff.split("/").collect::<Vec<&str>>()[0];
        let year: i32 = date_str.split_at(4).0.parse().unwrap();
        let month: u32 = date_str.split_at(4).1.split_at(2).0.parse().unwrap();
        let day: u32 = date_str.split_at(4).1.split_at(2).1.parse().unwrap();
        chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    /**
     * Get the index of a song in its album.
     */
    pub fn get_song_index(&self, album: &Album) -> Result<u16, Box<dyn Error>> {
        Ok(album
            .songs_ref()
            .iter()
            .position(|s: &SongSyn| s.cid_ref().eq(&self.cid))
            .unwrap() as u16 + 1)
    }

    /**
     * Get the index of a song in its album.
     * Same as get_song_index, but fetches the album details itself.
     */
    pub async fn fetch_song_index(&self) -> Result<u16, Box<dyn Error>> {
        let album: Album = fetch_album_details(&self.album_cid).await?;
        Ok(self.get_song_index(&album)?)
    }
}

impl SongSyn {
    /**
     * Fetch song details via the songs synopsis
     */
    pub async fn get_details(&self) -> Result<Song, Box<dyn Error>> {
        Ok(fetch_song_details(&self.cid).await?)
    }
}
impl Clone for SongSyn {
    fn clone(&self) -> Self {
        Self { cid: self.cid.clone(), name: self.name.clone(), album_cid: self.album_cid.clone(), artists: self.artists.clone() }
    }
}

/**
 * Fetch the list of songs currently available on MSR.
 * The list consists of song synopses only
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
