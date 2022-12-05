use std::collections::HashMap;

use reqwest::Client;

mod browser;
mod cookie;
mod cookies;
mod html;

/// A simplistic browser for creating requests on the web
pub struct Browser {
    cookies: Cookies,
    /// The main client for making requests
    client: Client,
    /// User Agent value of the browser, spoofed so the websites respond in expected way
    user_agent: String,
    /// Language the website is expected to return
    accept_language: String,
    /// Accepted mime types of the requests
    accept: String,
}
/// This is a simplistic way of storing cookie information to give them back to the websites as they expect.
/// This should be replaced with implementation from Reqwest framework sooner or later
#[derive(Default)]
pub struct Cookie {
    key: String,
    value: String,
    expires: String,
    path: String,
    domain: String,
    tags: Vec<String>,
    maps: HashMap<String, String>,
}
/// A collection of cookies. This should probably be replaced by implementation from Reqwest framework.
#[derive(Default)]
pub struct Cookies {
    cookies: Vec<Cookie>,
}

/// A trait for conveniently decoding special HTML character strings
pub trait HTMLDecodable {
    fn decode_html(&self) -> String;
}
