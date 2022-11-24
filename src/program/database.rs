use crate::{
    data::{Playlist, Video},
    enums::Error,
    file::File,
};

use super::Database;

impl Default for Database {
    fn default() -> Self {
        let playlists = Playlist::load_all();
        let videos = Video::load_all();
        Self { playlists, videos }
    }
}

impl Database {
    pub fn playlist_index(&self, id: &str) -> Option<usize> {
        self.playlists.iter().position(|x| x.id == id)
    }
    pub fn video_index(&self, id: &str) -> Option<usize> {
        self.videos.iter().position(|x| x.id == id)
    }
    pub fn add_playlist(&mut self, playlist: Playlist) -> Result<(), Error> {
        playlist.save()?;
        self.playlists.push(playlist);
        Ok(())
    }
    pub fn remove_playlist(&mut self, id: String) -> Result<(), Error> {
        let Some(i) = self.playlist_index(&id) else {
            return Err(Error::MissingID(id));
        };
        let p = self.playlists.remove(i);
        p.delete()?;
        Ok(())
    }
    pub fn get_playlist(&self, id: &str) -> Option<&Playlist> {
        let Some(i) = self.playlist_index(id) else {
            return None;
        };
        self.playlists.get(i)
    }
    pub fn update_playlist(&mut self, playlist: Playlist) {
        if let Some(i) = self.playlist_index(&playlist.id) {
            self.playlists.remove(i);
            self.playlists.insert(i, playlist);
        } else {
            self.playlists.push(playlist);
        }
    }
    pub fn update_video(&mut self, video: Video) {
        if let Some(i) = self.video_index(&video.id) {
            self.videos.remove(i);
            self.videos.insert(i, video);
        } else {
            self.videos.push(video);
        }
    }
    pub fn get_video(&self, id: &str) -> Option<&Video> {
        let Some(i) = self.video_index(id) else {
            return None;
        };
        self.videos.get(i)
    }
    pub fn get_videos_by_id(&self, ids: &[String]) -> Vec<&Video> {
        ids.iter()
            .map(|x| self.get_video(x))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    }
    pub fn add_video(&mut self, video: Video) -> Result<(), Error> {
        video.save()?;
        self.videos.push(video);
        Ok(())
    }
    pub fn get_missing_videos(&self, videos: &Vec<String>) -> Option<Vec<String>> {
        let v: Vec<String> = videos
            .iter()
            .filter(|x| self.videos.iter().any(|a| **x == a.id) == false)
            .map(|x| x.clone())
            .collect();
        if v.len() == 0 {
            None
        } else {
            Some(v)
        }
    }
}
