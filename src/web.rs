use std::collections::HashMap;

use reqwest::Client;

mod browser;
mod cookie;
mod cookies;

pub struct Browser {
    cookies: Cookies,
    client: Client,
    user_agent: String,
    accept_language: String,
    accept: String,
}
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
#[derive(Default)]
pub struct Cookies {
    cookies: Vec<Cookie>,
}
