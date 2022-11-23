
mod youtube;
use std::{sync::Arc, future::Future};

use iced::futures::lock::Mutex;

use crate::{web::Browser, data::Playlist, enums::Error};


type BrowserCarrier = Arc<Mutex<Browser>>;
#[derive(Default)]
pub struct ServiceProvider {
    browser: BrowserCarrier,
}

impl ServiceProvider {
    pub fn get_playlist(&self, url: String) -> impl Future<Output = Result<Playlist, Error>> {
        youtube::produce_playlist(self.browser.clone(), url)
    }
}
