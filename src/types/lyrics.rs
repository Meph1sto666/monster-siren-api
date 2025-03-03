use getters2::Getters;
use std::{error::Error, result::Result};

#[derive(serde::Serialize, Getters, Debug)]
pub struct SyncedLine {
    stamp: chrono::Duration,
    content: String,
}

#[derive(serde::Serialize, Getters)]
pub struct Lyrics {
    lines: Vec<SyncedLine>,
}

impl Lyrics {
    pub fn to_lrc(&self) -> String {
        self.lines
            .iter()
            .map(|v| {
                format!(
                    "[{:0>2}:{:0>2}.{:0<3}]{}",
                    v.stamp.num_minutes(),
                    v.stamp.num_seconds() % 60,
                    v.stamp.num_milliseconds() % 1000,
                    v.content
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub async fn from_lyrics_url(url: &str) -> Result<Lyrics, Box<dyn Error>> {
    let lines: Vec<SyncedLine> = reqwest::get(url)
        .await?
        .text()
        .await?
        .lines()
        .filter(|e| e.len() > 0)
        .map(|l| SyncedLine {
            stamp: sync_stamp_to_duration(l.split("]").collect::<Vec<&str>>()[0]).unwrap(),
            content: l.split("]").collect::<Vec<&str>>()[1].trim().to_owned(),
        })
        .collect::<Vec<SyncedLine>>();
    Ok(Lyrics { lines })
}

fn sync_stamp_to_duration(stamp: &str) -> Option<chrono::Duration> {
    let stamp: &str = stamp.trim_matches(|c: char| c == '[' || c == ']');
    let parts: Vec<&str> = stamp.split(":").collect();
    let minutes: i64 = parts[0].parse().ok()?;
    let total_millis: u32 = parts[1]
        .split(".")
        .enumerate()
        .map(|(i, v)| v.parse::<u32>().unwrap() * (1000_u32.pow(1 - i as u32)))
        .sum();
    Some(chrono::Duration::milliseconds(
        total_millis as i64 + minutes * 60000,
    ))
}
