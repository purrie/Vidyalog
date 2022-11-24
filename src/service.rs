
mod youtube;
use std::{sync::Arc, future::Future};

use iced::futures::lock::Mutex;

use crate::{web::Browser, data::{Playlist, Video}, enums::{Error, VideoService}};

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
    pub fn get_playlist(&self, browser: BrowserCarrier, url: String) -> impl Future<Output = Result<Playlist, Error>> {
        match self {
            VideoService::Unknown => youtube::procure_playlist(browser, url),
            VideoService::Youtube => youtube::procure_playlist(browser, url),
        }
    }
    pub fn get_video(&self, browser: BrowserCarrier, url: String) -> impl Future<Output = Result<Video, Error>> {
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