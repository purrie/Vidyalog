mod content_id;
mod youtube;

use std::{future::Future, marker::PhantomData, sync::Arc};

use iced::futures::lock::Mutex;
use serde::{Deserialize, Serialize};

use crate::{
    data::{Playlist, Thumbnail, Video},
    enums::{ContentType, Error, VideoService},
    web::Browser,
};

type BrowserCarrier = Arc<Mutex<Browser>>;

/// Provides an interface to download things from the internet
#[derive(Default)]
pub struct ServiceProvider {
    browser: BrowserCarrier,
}

impl ServiceProvider {
    /// Creates a Future returning a playlist. The service is automatically determined from the URL
    pub fn get_playlist(&self, url: String) -> impl Future<Output = Result<Playlist, Error>> {
        VideoService::match_by_url(&url).get_playlist(self.browser.clone(), url)
    }
    /// Creates a Future returning a video. The service is automatically determined from the URL
    pub fn get_video(&self, url: String) -> impl Future<Output = Result<Video, Error>> {
        VideoService::match_by_url(&url).get_video(self.browser.clone(), url)
    }
    /// Creates a Future returning a thumbnail. The service is automatically determined from the URL
    pub fn get_thumbnail(&self, url: String) -> impl Future<Output = Result<Thumbnail, Error>> {
        VideoService::match_by_url(&url).get_thumbnail(self.browser.clone(), url)
    }
}

/// Main implementation of the VideoService tags. New video services function calls should be added here as they are implemented.
impl VideoService {
    /// This function detects which service the link corresponds to
    fn match_by_url(_url: &str) -> Self {
        // TODO do a proper service detection
        VideoService::Unknown
    }
    /// The function links the tag to proper function for retrieving playlists from supported services.
    fn get_playlist(
        &self,
        browser: BrowserCarrier,
        url: String,
    ) -> impl Future<Output = Result<Playlist, Error>> {
        match self {
            VideoService::Unknown => youtube::procure_playlist(browser, url),
            VideoService::Youtube => youtube::procure_playlist(browser, url),
        }
    }
    /// The function links the tag to proper function for retrieving video information from supported services.
    fn get_video(
        &self,
        browser: BrowserCarrier,
        url: String,
    ) -> impl Future<Output = Result<Video, Error>> {
        match self {
            VideoService::Unknown => youtube::procure_video(browser, url),
            VideoService::Youtube => youtube::procure_video(browser, url),
        }
    }
    /// The function links the tag to proper function for retrieving a thumbnail image from supported services
    fn get_thumbnail(
        &self,
        browser: BrowserCarrier,
        url: String,
    ) -> impl Future<Output = Result<Thumbnail, Error>> {
        match self {
            VideoService::Unknown => youtube::procure_thumbnail(browser, url),
            VideoService::Youtube => youtube::procure_thumbnail(browser, url),
        }
    }
    /// This function turns the ID into a full URL of the service pointing to the playlist
    pub fn get_playlist_url(&self, id: &str) -> String {
        match self {
            VideoService::Unknown => youtube::get_playlist_url(id),
            VideoService::Youtube => youtube::get_playlist_url(id),
        }
    }
    /// This function turns the ID into a full URL of the service pointing to a video
    pub fn get_video_url(&self, id: &str) -> String {
        match self {
            VideoService::Unknown => youtube::get_video_url(id),
            VideoService::Youtube => youtube::get_video_url(id),
        }
    }
    /// This function turns the ID into a full URL of the service pointing to a thumbnail image
    pub fn get_thumbnail_url(&self, id: &str) -> String {
        match self {
            VideoService::Unknown => youtube::get_thumbnail_url(id),
            VideoService::Youtube => youtube::get_thumbnail_url(id),
        }
    }
}

/// Content Identifier works as a pointer to the appropriate content data by its type
#[derive(Default, PartialEq, Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct ContentIdentifier<T> {
    /// The service associated with the identifier
    pub service: VideoService,
    /// the id of the content on the service
    pub id: String,
    /// The type of the content this identifier points to
    pub content: ContentType,
    /// Phantom Data for locking the identifier to a specific type
    pd: PhantomData<T>,
}

/// Trait for marking a struct as identifiable content from a specific service. It provides a function to retrieve an id pointer to this content
pub trait ContentID: PartialEq {
    /// Retrieves a pointer that can be used to point to and identify this content
    fn get_content_id(&self) -> ContentIdentifier<Self>
    where
        Self: Sized;
    /// Retrieves an URL from where this content can be obtained
    fn get_content_url_path(&self) -> String
    where
        Self: Sized,
    {
        self.get_content_id().get_url()
    }
}
