pub struct Clip {
    pub start_time: usize,
    pub end_time: usize,
    pub song_id: usize,
}

impl Clip {
    pub fn new() -> Self {
        Clip {
            start_time: 0,
            end_time: 0,
            song_id: 0,
        }
    }
    pub fn from(start_time: usize, end_time: usize, song_id: usize) -> Self {
        Clip {
            start_time,
            end_time,
            song_id,
        }
    }
}
