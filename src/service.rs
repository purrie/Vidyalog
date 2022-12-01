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

#[derive(Default)]
pub struct ServiceProvider {
    browser: BrowserCarrier,
}

impl ServiceProvider {
    pub fn get_playlist(&self, url: String) -> impl Future<Output = Result<Playlist, Error>> {
        VideoService::match_by_url(&url).get_playlist(self.browser.clone(), url)
    }
    pub fn get_video(&self, url: String) -> impl Future<Output = Result<Video, Error>> {
        VideoService::match_by_url(&url).get_video(self.browser.clone(), url)
    }
    pub fn get_thumbnail(&self, url: String) -> impl Future<Output = Result<Thumbnail, Error>> {
        VideoService::match_by_url(&url).get_thumbnail(self.browser.clone(), url)
    }
}

impl VideoService {
    fn match_by_url(_url: &str) -> Self {
        // TODO do a proper service detection
        VideoService::Unknown
    }
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
    pub fn get_playlist_url(&self, id: &str) -> String {
        match self {
            VideoService::Unknown => youtube::get_playlist_url(id),
            VideoService::Youtube => youtube::get_playlist_url(id),
        }
    }
    pub fn get_video_url(&self, id: &str) -> String {
        match self {
            VideoService::Unknown => youtube::get_video_url(id),
            VideoService::Youtube => youtube::get_video_url(id),
        }
    }
    pub fn get_thumbnail_url(&self, id: &str) -> String {
        match self {
            VideoService::Unknown => youtube::get_thumbnail_url(id),
            VideoService::Youtube => youtube::get_thumbnail_url(id),
        }
    }
}

#[derive(Default, PartialEq, Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct ContentIdentifier<T> {
    pub service: VideoService,
    pub id: String,
    pub content: ContentType,
    pd: PhantomData<T>,
}

pub trait ContentID: PartialEq {
    fn get_content_id(&self) -> ContentIdentifier<Self>
    where
        Self: Sized;
    fn get_content_url_path(&self) -> String
    where
        Self: Sized,
    {
        self.get_content_id().get_url()
    }
}
