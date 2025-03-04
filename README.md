# API for Monster Siren Records

This is a unofficial Rust wrapper for Monster Siren's API

## Cargo

`cargo add msr-api`

## Examples


### Printing lyrics of a song
```rust
use msr_api::types::{lyrics::{from_lyrics_url, Lyrics}, song::{fetch_song_details, Song}};
use tokio;

#[tokio::main]
async fn main() {
	let song: Song = fetch_song_details("514530").await.unwrap();
	let lyrics: Lyrics = from_lyrics_url(song.lyrics_url_ref().to_owned().unwrap().as_str()).await.unwrap();
}
```

### Fetch song details and index
```rust
use msr_api::types::{album::{fetch_album_details, Album}, song::fetch_song_list};
use tokio;

#[tokio::main]
async fn main() {
    let song_list: Vec<msr_api::types::song::SongSyn> = fetch_song_list().await.unwrap();
    for s in song_list.iter().cloned() {
        let album: Album = fetch_album_details(&s.clone().album_cid_ref().to_owned().unwrap()).await.unwrap();
        let idx: u16 = s.get_details().await.unwrap().get_song_index(&album).unwrap();
        println!("[{}] {} in {}", s.cid_ref(), idx, album.name_ref())
    }
}
```

Contributions are welcome, just make sure to install the --dev dependencies for testing

> This project is not affiliated with Hypergryph, Studio Montagne, Yostar or Gryphline in any ways.