use super::error::{AppError, AppResult};
use std::collections::VecDeque;

pub struct MediaPlayer {
    playlist: VecDeque<MediaItem>,
    current_item: Option<MediaItem>,
    state: PlaybackState,
    volume: f32,
    config: MediaPlayerConfig,
}

struct MediaItem {
    path: String,
    metadata: MediaMetadata,
    duration: Duration,
}

struct MediaMetadata {
    title: String,
    artist: String,
    album: String,
    year: Option<u16>,
    genre: Option<String>,
}

enum PlaybackState {
    Stopped,
    Playing,
    Paused,
    Loading,
    Error(String),
}

impl MediaPlayer {
    pub fn new(config: MediaPlayerConfig) -> Self {
        Self {
            playlist: VecDeque::new(),
            current_item: None,
            state: PlaybackState::Stopped,
            volume: 1.0,
            config,
        }
    }

    pub fn play(&mut self) -> AppResult<()> {
        match self.state {
            PlaybackState::Stopped | PlaybackState::Paused => {
                if let Some(item) = &self.current_item {
                    self.start_playback(item)?;
                    self.state = PlaybackState::Playing;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn add_to_playlist(&mut self, path: &str) -> AppResult<()> {
        let metadata = self.load_metadata(path)?;
        let item = MediaItem {
            path: path.to_string(),
            metadata,
            duration: Duration::from_secs(0), // Placeholder
        };
        self.playlist.push_back(item);
        Ok(())
    }
} 