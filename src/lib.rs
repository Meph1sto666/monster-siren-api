pub mod types;

#[cfg(test)]
mod tests {
	use crate::types::song::{fetch_autoplay_song_id, fetch_song_details, fetch_song_list, Song};
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
}
