use crate::data::Track;
use plist::Value;
use std::error::Error;
use std::fs;

pub fn read_xml_file(path: &str) -> Result<Value, Box<dyn Error>> {
    let xml_content = fs::read_to_string(path)?;
    let plist = Value::from_reader_xml(xml_content.as_bytes())?;
    Ok(plist)
}

pub fn extract_tracks(plist: &Value) -> Vec<Track> {
    let mut tracks_list = Vec::new();

    // "Tracks" の部分が Dictionary であることを確認して取得
    if let Some(tracks_dict) = plist
        .as_dictionary()
        .and_then(|dict| dict.get("Tracks"))
        .and_then(|v| v.as_dictionary())
    {
        for (track_id, track_info) in tracks_dict {
            // Track の各フィールドを取得
            if let Some(track_details) = track_info.as_dictionary() {
                let track = Track {
                    track_id: track_id.parse().unwrap_or(0),
                    name: track_details
                        .get("Name")
                        .and_then(|v| v.as_string())
                        .map(String::from)
                        .unwrap_or_else(|| String::from("Unknown")),
                    artist: track_details
                        .get("Artist")
                        .and_then(|v| v.as_string())
                        .map(String::from),
                    album: track_details
                        .get("Album")
                        .and_then(|v| v.as_string())
                        .map(String::from),
                    total_time: track_details
                        .get("Total Time")
                        .and_then(|v| v.as_signed_integer())
                        .map(|v| v as i32),
                    play_count: track_details
                        .get("Play Count")
                        .and_then(|v| v.as_signed_integer())
                        .map(|v| v as i32),
                };
                tracks_list.push(track);
            }
        }
    }

    tracks_list
}

// トラックリストを出力するヘルパー関数
pub fn print_tracks(tracks: &[Track]) {
    for track in tracks {
        println!("Track ID: {}", track.track_id);
        println!("  Name: {}", track.name);
        if let Some(artist) = &track.artist {
            println!("  Artist: {}", artist);
        }
        if let Some(album) = &track.album {
            println!("  Album: {}", album);
        }
        if let Some(total_time) = track.total_time {
            println!("  Total Time: {} ms", total_time);
        }
        if let Some(play_count) = track.play_count {
            println!("  Play Count: {}", play_count);
        }
    }
}
