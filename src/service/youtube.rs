use htmlentity::entity::decode;
use regex::{Captures, Regex};

use crate::{data::Playlist, enums::Error};

use super::BrowserCarrier;

pub async fn produce_playlist(
    browser: BrowserCarrier,
    url: String,
) -> Result<Playlist, Error> {
    let mut browser = browser.lock().await;

        let id = {
            let id = url
                .split(['?', '&'])
                .filter(|x| x.starts_with("list"))
                .map(|x| x.split('=').fold(String::new(), |_, i| i.to_string()))
                .fold(String::new(), |_, i| i);
            if id.len() == 0 {
                return Err(Error::InvalidPlaylistURL(url));
            }
            id
        };
    let url = format!("https://www.youtube.com/playlist?list={}", id);
    let res = browser.open(&url).await?;

    if res.status() == 200 {
        println!("Success");
        let body = res.text().await?;
        let body = {
            let mut body = body.replace('\n', "");
            let regex = Regex::new(r"(</.+?>)").unwrap();
            body = regex
                .replace_all(&body, |x: &Captures| {
                    let c = x.get(1).unwrap().as_str();
                    format!("{}\n", c)
                })
                .to_string();
            body
        };
        let title = {
            let regex = Regex::new(r"<title>(.+?)</title>").unwrap();
            if let Some(t) = regex.captures(&body) {
                let t = t.get(1).unwrap().as_str();
                let t = decode(&t).iter().fold(String::new(), |en, it| format!("{}{}", en, it));
                // TODO decode the encoded stuff properly
                println!("Found title: {:?}", t);
                t.to_string()
            } else {
                return Err(Error::IncompleteResponse);
            }
        };
        let videos = {
            let mut videos = Vec::new();
            let playlist_data = body
                .lines()
                .filter(|x| x.starts_with("<script"))
                .filter(|x| x.contains("ytInitialData"))
                .fold(String::new(), |i, x| format!("{}\n{}", i, x));
            let regex =
                Regex::new(r#""watchEndpoint":\{"videoId":"(.+?)",.+?"index":(\d+),"#).unwrap();
            regex.captures_iter(&playlist_data).for_each(|x| {
                let id = x.get(1).unwrap().as_str().to_string();
                if videos.contains(&id) == false {
                    videos.push(id);
                }
            });
            if videos.len() == 0 {
                return Err(Error::IncompleteResponse);
            }
            videos
        };
        // to get the playlist thumbnail, use playlistVideoThumbnailRenderer lookup
        Ok(Playlist {
            id,
            url,
            title,
            videos,
        })
    } else {
        Err(Error::ReqwestError(format!(
            "Network request failed: {}",
            res.status()
        )))
    }
}
