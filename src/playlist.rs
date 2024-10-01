use crate::{clip::Clip, song::Song};

pub struct Playlist {
    pub songs: Vec<Song>,
    pub clips: Vec<Clip>,
}

impl Playlist {
    pub fn new() -> Self {
        Playlist {
            songs: Vec::new(),
            clips: Vec::new(),
        }
    }
    pub fn add_source(&mut self, path: String) {
        self.songs.push(Song::from(path));
    }
    pub fn create_clip(&mut self, song_id: usize, start_time: usize, end_time: usize) {
        self.clips.push(Clip::from(start_time, end_time, song_id));
    }
    pub fn print_clips(&self) {
        for clip in &self.clips {
            println!("{} ({} - {})", self.songs.get(clip.song_id).unwrap().path, clip.start_time, clip.end_time);
        }
    }
}
