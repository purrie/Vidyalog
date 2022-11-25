mod content_id;
mod youtube;

use std::{future::Future, marker::PhantomData, sync::Arc};

use iced::futures::lock::Mutex;
use serde::{Deserialize, Serialize};

use crate::{
    data::{Playlist, Video},
    enums::{Error, VideoService},
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
    pub fn get_browser(&self) -> BrowserCarrier {
        self.browser.clone()
    }
}

impl VideoService {
    pub fn match_by_url(_url: &str) -> Self {
        // TODO do a proper service detection
        VideoService::Unknown
    }
    pub fn get_playlist(
        &self,
        browser: BrowserCarrier,
        url: String,
    ) -> impl Future<Output = Result<Playlist, Error>> {
        match self {
            VideoService::Unknown => youtube::procure_playlist(browser, url),
            VideoService::Youtube => youtube::procure_playlist(browser, url),
        }
    }
    pub fn get_video(
        &self,
        browser: BrowserCarrier,
        url: String,
    ) -> impl Future<Output = Result<Video, Error>> {
        match self {
            VideoService::Unknown => youtube::procure_video(browser, url),
            VideoService::Youtube => youtube::procure_video(browser, url),
        }
    }
    pub fn get_video_url(&self, id: &str) -> String {
        match self {
            VideoService::Unknown => youtube::get_video_url(id),
            VideoService::Youtube => youtube::get_video_url(id),
        }
    }
}

#[derive(Default, PartialEq, Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct ContentIdentifier<T> {
    service: VideoService,
    id: String,
    pd: PhantomData<T>,
}

pub trait ContentID: PartialEq {
    fn get_content_id(&self) -> ContentIdentifier<Self>
    where
        Self: Sized;
}
