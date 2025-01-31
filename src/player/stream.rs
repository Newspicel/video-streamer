#[cfg(feature = "ssr")]
use std::fs::File;
#[cfg(feature = "ssr")]
use std::io::{Read, Seek, SeekFrom};
use std::collections::HashMap;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
const CHUNK_SIZE: usize = 1_000_000; // 1MB chunks

#[cfg(feature = "ssr")]
fn get_video_path(video_id: &str) -> Option<String> {
    let video_map: HashMap<&str, &str> = HashMap::from([
        ("video1", "videos/video1.mp4"),
        ("video2", "videos/video2.mp4"),
        ("video3", "videos/video3.mp4"),
    ]);
    video_map.get(video_id).map(|path| path.to_string())
}

#[server(StreamVideo)]
pub async fn stream_video(video_id: String, start_byte: Option<u64>) -> Result<Vec<u8>, ServerFnError> {
    let video_path = match get_video_path(&video_id) {
        Some(path) => path,
        None => return Err(ServerFnError::new("Video not found")),
    };

    let mut file = File::open(video_path).map_err(|_| ServerFnError::new("Cannot open file"))?;
    let metadata = file
        .metadata()
        .map_err(|_| ServerFnError::new("Cannot get metadata"))?;
    let file_size = metadata.len();

    let start = start_byte.unwrap_or(0);
    let end = (start + CHUNK_SIZE as u64).min(file_size - 1);
    let chunk_size = (end - start + 1) as usize;

    let mut buffer = vec![0; chunk_size];
    file.seek(SeekFrom::Start(start))
        .map_err(|_| ServerFnError::new("Seek failed"))?;
    file.read_exact(&mut buffer)
        .map_err(|_| ServerFnError::new("Read failed"))?;

    Ok(buffer)
}