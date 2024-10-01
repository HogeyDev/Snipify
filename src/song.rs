pub struct Song {
    pub path: String,
}

impl Song {
    pub fn from(path: String) -> Self {
        Song {
            path,
        }
    }
}
