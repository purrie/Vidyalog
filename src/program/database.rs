use std::iter::Iterator;

use crate::{
    data::{Playlist, PlaylistFeed, Thumbnail, Video},
    enums::{Error, VideoStatus},
    file::File,
    service::{ContentID, ContentIdentifier},
};

use super::Database;

impl Default for Database {
    /// By default, the database will load all the data from the drive.
    ///
    /// This is a convenience as to not require any additional steps in the initialization since it isn't expected to create more than one database
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
    /// Returns index of the playlist by its ID or None if it haven't been found
    pub fn playlist_index(&self, id: &ContentIdentifier<Playlist>) -> Option<usize> {
        self.playlists.iter().position(|x| id.identify(x))
    }
    /// Returns index of a video by its ID or None if it haven't been found
    pub fn video_index(&self, id: &ContentIdentifier<Video>) -> Option<usize> {
        self.videos.iter().position(|x| id.identify(x))
    }
    /// Returns index of the thumbnail by its ID or None if it haven't been found
    pub fn thumbnail_index(&self, id: &ContentIdentifier<Thumbnail>) -> Option<usize> {
        self.thumbnails.iter().position(|x| id.identify(x))
    }
    /// Adds a new playlist to the database
    ///
    /// If the playlist by the same ID exists in the database, it will be non-destructively updated instead.
    /// The function also saves the playlist data to drive.
    ///
    /// # Errors
    /// The function will error out if the playlist couldn't be saved to drive.
    pub fn add_playlist(&mut self, playlist: Playlist) -> Result<(), Error> {
        if let Some(i) = self.playlist_index(&playlist.get_content_id()) {
            let pl = self.playlists.get_mut(i).unwrap();
            pl.update(playlist);
            pl.save_content()?;
        } else {
            playlist.save_content()?;
            self.playlists.push(playlist);
        }
        Ok(())
    }
    /// Removes the playlist by its ID from the database
    ///
    /// The playlist file will also be removed from drive.
    ///
    /// # Errors
    /// The function will return an error if it didn't successfully delete the file.
    /// If the playlist couldn't be found, error will be returned as well.
    pub fn remove_playlist(&mut self, id: &ContentIdentifier<Playlist>) -> Result<(), Error> {
        let Some(i) = self.playlist_index(id) else {
            return Err(Error::Unknown);
        };
        let p = self.playlists.remove(i);
        p.delete()?;
        Ok(())
    }
    /// The function will filter out every playlist that has unwatched videos and return them as a list of PlaylistFeed
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
    /// Returns a playlist by its ID or None if it couldn't be located.
    pub fn get_playlist(&self, id: &ContentIdentifier<Playlist>) -> Option<&Playlist> {
        let Some(i) = self.playlist_index(id) else {
            return None;
        };
        self.playlists.get(i)
    }
    /// Returns a mutable playlist by its ID or None if it couldn't be located
    pub fn get_playlist_mut(&mut self, id: &ContentIdentifier<Playlist>) -> Option<&mut Playlist> {
        let Some(i) = self.playlist_index(id) else {
            return None;
        };
        self.playlists.get_mut(i)
    }
    /// Returns a video by its ID or None if it couldn't be located
    pub fn get_video(&self, id: &ContentIdentifier<Video>) -> Option<&Video> {
        let Some(i) = self.video_index(id) else {
            return None;
        };
        self.videos.get(i)
    }
    /// Returns a mutable video by its ID or None if it couldn't be located
    pub fn get_video_mut(&mut self, id: &ContentIdentifier<Video>) -> Option<&mut Video> {
        let Some(i) = self.video_index(id) else {
            return None;
        };
        self.videos.get_mut(i)
    }
    /// Returns a list of videos by a list of IDs
    pub fn get_videos_by_id(&self, ids: &[ContentIdentifier<Video>]) -> Vec<&Video> {
        ids.iter()
            .map(|x| self.get_video(x))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    }
    /// Adds a new video to the database
    ///
    /// If a video by the same ID exists already in the database then it will be updated non-destructively instead.
    /// The video data will then be saved to the drive.
    ///
    /// # Errors
    /// The function will return an error if the video failed to save to the drive
    pub fn add_video(&mut self, video: Video) -> Result<(), Error> {
        if let Some(i) = self.video_index(&video.get_content_id()) {
            let v = self.videos.get_mut(i).unwrap();
            v.update(video);
            v.save_content()?;
        } else {
            video.save_content()?;
            self.videos.push(video);
        }
        Ok(())
    }
    /// Returns a list of videos that are not in the database when they are present in the provided list or None if all the videos exist in the database
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
    /// Returns a filtered thumbnail iterator with the predicate applied to it
    pub fn filter_thumbnails_mut<T>(&mut self, pred: T) -> impl Iterator<Item = &mut Thumbnail>
    where
        T: FnMut(&&mut Thumbnail) -> bool,
    {
        self.thumbnails.iter_mut().filter(pred)
    }
    /// Returns an image handle of a thumbnail by its ID or None if the thumbnail lacks an image
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
    /// Adds a thumbnail to the database
    ///
    /// If the thumbnail already exists, it will be updated non-destructively instead
    /// The thumbnail is then saved to the drive.
    ///
    /// # Errors
    /// The function will error out if the update failed when the thumbnail already exists in the database.
    /// Also, the function will fail if the thumbnail didn't save properly.
    pub fn add_thumbnail(&mut self, thumb: Thumbnail) -> Result<(), Error> {
        if let Some(i) = self.thumbnail_index(&thumb.get_content_id()) {
            let t = self.thumbnails.get_mut(i).unwrap();
            t.update(thumb)?;
            t.save_content()?;
        } else {
            thumb.save_content()?;
            self.thumbnails.push(thumb);
        }
        Ok(())
    }
}
