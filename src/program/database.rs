use std::iter::Iterator;

use crate::{
    data::{Playlist, PlaylistFeed, Thumbnail, Video},
    enums::{Error, VideoStatus},
    file::File,
    service::{ContentID, ContentIdentifier},
};

use super::Database;

impl Default for Database {
    fn default() -> Self {
        let playlists = Playlist::load_all();
        let videos = Video::load_all();
        let thumbnails = Thumbnail::load_all();
        Self {
            playlists,
            videos,
            thumbnails,
        }
    }
}

impl Database {
    pub fn playlist_index(&self, id: &ContentIdentifier<Playlist>) -> Option<usize> {
        self.playlists.iter().position(|x| id.identify(x))
    }
    pub fn video_index(&self, id: &ContentIdentifier<Video>) -> Option<usize> {
        self.videos.iter().position(|x| id.identify(x))
    }
    pub fn thumbnail_index(&self, id: &ContentIdentifier<Thumbnail>) -> Option<usize> {
        self.thumbnails.iter().position(|x| id.identify(x))
    }
    pub fn add_playlist(&mut self, playlist: Playlist) -> Result<(), Error> {
        if let Some(i) = self.playlist_index(&playlist.get_content_id()) {
            let pl = self.playlists.get_mut(i).unwrap();
            pl.update(playlist);
            pl.save()?;
        } else {
            playlist.save()?;
            self.playlists.push(playlist);
        }
        Ok(())
    }
    pub fn remove_playlist(&mut self, id: &ContentIdentifier<Playlist>) -> Result<(), Error> {
        let Some(i) = self.playlist_index(id) else {
            return Err(Error::Unknown);
        };
        let p = self.playlists.remove(i);
        p.delete()?;
        Ok(())
    }
    pub fn get_fresh_playlists(&self) -> Vec<PlaylistFeed> {
        self.playlists
            .iter()
            .filter(|x| x.tracked)
            .filter_map(|p| {
                if let Some(v) = p
                    .videos
                    .iter()
                    .filter_map(|id| self.get_video(id))
                    .find(|v| v.status != VideoStatus::Watched)
                {
                    Some(PlaylistFeed {
                        playlist: p,
                        latest: v,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn get_playlist(&self, id: &ContentIdentifier<Playlist>) -> Option<&Playlist> {
        let Some(i) = self.playlist_index(id) else {
            return None;
        };
        self.playlists.get(i)
    }
    pub fn get_playlist_mut(&mut self, id: &ContentIdentifier<Playlist>) -> Option<&mut Playlist> {
        let Some(i) = self.playlist_index(id) else {
            return None;
        };
        self.playlists.get_mut(i)
    }
    pub fn get_video(&self, id: &ContentIdentifier<Video>) -> Option<&Video> {
        let Some(i) = self.video_index(id) else {
            return None;
        };
        self.videos.get(i)
    }
    pub fn get_video_mut(&mut self, id: &ContentIdentifier<Video>) -> Option<&mut Video> {
        let Some(i) = self.video_index(id) else {
            return None;
        };
        self.videos.get_mut(i)
    }
    pub fn get_videos_by_id(&self, ids: &[ContentIdentifier<Video>]) -> Vec<&Video> {
        ids.iter()
            .map(|x| self.get_video(x))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    }
    pub fn add_video(&mut self, video: Video) -> Result<(), Error> {
        if let Some(i) = self.video_index(&video.get_content_id()) {
            let v = self.videos.get_mut(i).unwrap();
            v.update(video);
            v.save()?;
        } else {
            video.save()?;
            self.videos.push(video);
        }
        Ok(())
    }
    pub fn get_missing_videos(
        &self,
        videos: &Vec<ContentIdentifier<Video>>,
    ) -> Option<Vec<ContentIdentifier<Video>>> {
        let v: Vec<ContentIdentifier<Video>> = videos
            .iter()
            .filter(|x| self.videos.iter().any(|a| x.identify(a)) == false)
            .map(|x| x.clone())
            .collect();
        if v.len() == 0 {
            None
        } else {
            Some(v)
        }
    }
    pub fn filter_thumbnails_mut<T>(&mut self, pred: T) -> impl Iterator<Item = &mut Thumbnail>
    where
        T: FnMut(&&mut Thumbnail) -> bool,
    {
        self.thumbnails.iter_mut().filter(pred)
    }
    pub fn get_thumbnail_image(
        &self,
        id: &ContentIdentifier<Thumbnail>,
    ) -> Option<iced::widget::image::Handle> {
        if let Some(i) = self.thumbnail_index(id) {
            self.thumbnails.get(i).unwrap().get_image()
        } else {
            None
        }
    }
    pub fn add_thumbnail(&mut self, thumb: Thumbnail) -> Result<(), Error> {
        if let Some(i) = self.thumbnail_index(&thumb.get_content_id()) {
            let t = self.thumbnails.get_mut(i).unwrap();
            t.update(thumb)?;
            t.save()?;
        } else {
            thumb.save()?;
            self.thumbnails.push(thumb);
        }
        Ok(())
    }
}
