use crate::data::Track;
use sqlx::MySqlPool;

pub async fn insert_tracks(pool: &MySqlPool, tracks: Vec<Track>) -> Result<(), sqlx::Error> {
    for track in tracks {
        // Key-Valueのタプルからtrackのみを扱う
        sqlx::query!(
            r#"
          INSERT INTO tracks (track_id, name, artist, album, total_time, play_count)
          VALUES (?, ?, ?, ?, ?, ?)
          ON DUPLICATE KEY UPDATE
              name = VALUES(name),
              artist = VALUES(artist),
              album = VALUES(album),
              total_time = VALUES(total_time),
              play_count = VALUES(play_count)
          "#,
            track.track_id,
            track.name,
            track.artist,
            track.album,
            track.total_time,
            track.play_count.unwrap_or(0)
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

#[derive(Debug)]
pub struct DatabaseError;
impl warp::reject::Reject for DatabaseError {}
