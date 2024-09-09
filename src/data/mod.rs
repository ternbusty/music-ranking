pub mod database;
pub mod parse;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)] // ここで Serialize を追加
pub struct Track {
    pub track_id: i32,
    pub name: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub total_time: Option<i32>, // ミリ秒単位
    pub play_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TrackWithPlaytime {
    track_id: i32,
    name: String,
    artist: Option<String>,
    album: Option<String>,
    total_time: Option<i32>,
    play_count: Option<i32>,
    playtime: Option<i64>, // 再生時間の計算結果を格納するフィールド
}
