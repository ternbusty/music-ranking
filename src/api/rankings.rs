use crate::data;
use sqlx::MySqlPool;
use warp::Filter;

pub fn create_routes(
    pool: MySqlPool,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let track_playtime_ranking = warp::path("rankings")
        .and(warp::path("tracks"))
        .and(warp::path("playtime"))
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handle_track_playtime_ranking);

    let track_playcount_ranking = warp::path("rankings")
        .and(warp::path("tracks"))
        .and(warp::path("playcount"))
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handle_track_playcount_ranking);

    let album_playtime_ranking = warp::path("rankings")
        .and(warp::path("albums"))
        .and(warp::path("playtime"))
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handle_album_playtime_ranking);

    let album_playcount_ranking = warp::path("rankings")
        .and(warp::path("albums"))
        .and(warp::path("playcount"))
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handle_album_playcount_ranking);

    let artist_playtime_ranking = warp::path("rankings")
        .and(warp::path("artists"))
        .and(warp::path("playtime"))
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handle_artist_playtime_ranking);

    let artist_playcount_ranking = warp::path("rankings")
        .and(warp::path("artists"))
        .and(warp::path("playcount"))
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handle_artist_playcount_ranking);

    track_playtime_ranking
        .or(track_playcount_ranking)
        .or(album_playtime_ranking)
        .or(album_playcount_ranking)
        .or(artist_playtime_ranking)
        .or(artist_playcount_ranking)
}

fn with_db(
    pool: MySqlPool,
) -> impl Filter<Extract = (MySqlPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

// 曲ごとの再生時間ランキング
async fn handle_track_playtime_ranking(
    pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let track_with_playtime = sqlx::query!(
        r#"
        SELECT track_id, name, artist, album, total_time, play_count, 
               (play_count * total_time) AS playtime
        FROM tracks
        ORDER BY playtime DESC
        LIMIT 10
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| warp::reject::custom(data::database::DatabaseError))?;

    Ok(warp::reply::json(&track_with_playtime))
}

// 曲ごとの再生回数ランキング
async fn handle_track_playcount_ranking(
    pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let tracks = sqlx::query!(
        r#"
        SELECT track_id, name, artist, album, total_time, play_count
        FROM tracks
        ORDER BY play_count DESC
        LIMIT 10
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| warp::reject::custom(data::database::DatabaseError))?;

    Ok(warp::reply::json(&tracks))
}

// アルバムごとの再生時間ランキング
async fn handle_album_playtime_ranking(
    pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let albums = sqlx::query!(
        r#"
        SELECT album, SUM(play_count) AS total_play_count, 
               SUM(play_count * total_time) AS total_playtime
        FROM tracks
        GROUP BY album
        ORDER BY total_playtime DESC
        LIMIT 10
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| warp::reject::custom(data::database::DatabaseError))?;

    Ok(warp::reply::json(&albums))
}

// アルバムごとの再生回数ランキング
async fn handle_album_playcount_ranking(
    pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let albums = sqlx::query!(
        r#"
        SELECT album, SUM(play_count) AS total_play_count
        FROM tracks
        GROUP BY album
        ORDER BY total_play_count DESC
        LIMIT 10
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| warp::reject::custom(data::database::DatabaseError))?;

    Ok(warp::reply::json(&albums))
}

// アーティストごとの再生時間ランキング
async fn handle_artist_playtime_ranking(
    pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let artists = sqlx::query!(
        r#"
        SELECT artist, SUM(play_count) AS total_play_count, 
               SUM(play_count * total_time) AS total_playtime
        FROM tracks
        GROUP BY artist
        ORDER BY total_playtime DESC
        LIMIT 10
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| warp::reject::custom(data::database::DatabaseError))?;

    Ok(warp::reply::json(&artists))
}

// アーティストごとの再生回数ランキング
async fn handle_artist_playcount_ranking(
    pool: MySqlPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let artists = sqlx::query!(
        r#"
        SELECT artist, SUM(play_count) AS total_play_count
        FROM tracks
        GROUP BY artist
        ORDER BY total_play_count DESC
        LIMIT 10
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| warp::reject::custom(data::database::DatabaseError))?;

    Ok(warp::reply::json(&artists))
}
