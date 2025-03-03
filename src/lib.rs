pub mod types;

#[cfg(test)]
mod tests {
    use crate::types::{
        album::{fetch_album_details, fetch_album_list, Album}, lyrics::{from_lyrics_url, Lyrics}, song::{fetch_autoplay_song_id, fetch_song_details, fetch_song_list, Song}
    };
    use tokio;

    #[tokio::test]
    async fn test_fetch_song_list() {
        let song_list: Vec<crate::types::song::SongSyn> = fetch_song_list().await.unwrap();
        assert!(song_list.len() > 0);
    }
    #[tokio::test]
    async fn test_fetch_autoplay_song_id() {
        let auto_play_song: String = fetch_autoplay_song_id().await.unwrap();
        assert!(auto_play_song.eq("048794"));
    }

    #[tokio::test]
    async fn test_fetch_song_details() {
        let song: Song = fetch_song_details("880318").await.unwrap();
        assert!(song.cid_ref().eq("880318"));
        assert!(song.name_ref().eq("Protocol"));
        assert!(song.album_cid_ref().eq("7769"));
    }

    #[tokio::test]
    async fn test_fetch_album_details() {
        let album: Album = fetch_album_details("7769").await.unwrap();
        assert!(album.cid_ref().eq("7769"));
        assert!(album.name_ref().eq("卫戍协议OST"));
        assert!(album.songs_ref().len() == 2);
    }
    #[tokio::test]
    async fn test_fetch_album_list() {
        let albums: Vec<crate::types::album::AlbumSyn> = fetch_album_list().await.unwrap();
        let a0: &crate::types::album::AlbumSyn = albums.get(0).unwrap();
        assert!(albums.len() >= 215);
        assert!(a0.cid_ref().eq("4504"))
    }
    #[tokio::test]
    async fn test_lyrics_from_url() {
        let lyrics: Lyrics = from_lyrics_url("https://web.hycdn.cn/siren/lyric/20230427/673ac618161e937954f4b8fc93c346fd.lrc").await.unwrap();
        assert!(lyrics.lines_ref()[0].content_ref().eq("Waiting by the window to revel in the last light of day"));
        assert!(lyrics.lines_ref()[1].content_ref().eq("Hoping that these empty halls are here to guide my way"));
        println!("{}", lyrics.to_lrc());
    }
    
    
    #[tokio::test]
    async fn test_get_release_date() {
        let song: Song = fetch_song_details("880318").await.unwrap();
        let date: chrono::NaiveDate = song.get_release_date();
        assert!(date.eq(&chrono::NaiveDate::from_ymd_opt(2024, 11, 15).unwrap()))
    }
}
