use regex::{Captures, Regex};

use crate::{
    data::{Playlist, Video},
    enums::{Error, VideoService, VideoStatus}, web::HTMLDecodable,
};

use super::BrowserCarrier;

pub async fn procure_playlist(browser: BrowserCarrier, url: String) -> Result<Playlist, Error> {
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
                t.decode_html()
            } else {
                return Err(Error::IncompleteResponse);
            }
        };
        let playlist_data = body
            .lines()
            .filter(|x| x.starts_with("<script"))
            .filter(|x| x.contains("ytInitialData"))
            .fold(String::new(), |i, x| format!("{}\n{}", i, x));

        let videos = {
            let mut videos = Vec::new();
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
        let description = {
            let regex = Regex::new(r#""descriptionText":\{"simpleText":"(.+?)"\}"#).unwrap();
            if let Some(c) = regex.captures(&playlist_data) {
                c.get(1).unwrap().as_str().decode_html()
            } else {
                String::new()
            }
        };
        // to get the playlist thumbnail, use playlistVideoThumbnailRenderer lookup
        Ok(Playlist {
            id,
            url,
            title,
            videos,
            description,
            source: VideoService::Youtube,
        })
    } else {
        Err(Error::ReqwestError(format!(
            "Network request failed: {}",
            res.status()
        )))
    }
}

pub async fn procure_video(browser: BrowserCarrier, url: String) -> Result<Video, Error> {
    let mut browser = browser.lock().await;

    let id = {
        let id = url
            .split(['?', '&'])
            .filter(|x| x.starts_with("v="))
            .map(|x| x.split('=').fold(String::new(), |_, i| i.to_string()))
            .fold(String::new(), |_, i| i);
        if id.len() == 0 {
            return Err(Error::InvalidVideoURL(url));
        }
        id
    };
    let url = get_video_url(&id);
    let res = browser.open(&url).await?;

    if res.status() == 200 {
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

        let video_details = {
            let line = body
                .lines()
                .filter(|x| x.contains(r#""videoDetails":{"videoId":"#))
                .fold(String::new(), |_, x| x.to_string());
            let regex =
                Regex::new(r#""videoDetails":\{"videoId":".+?",(.+?)trackingParams"#).unwrap();
            if let Some(c) = regex.captures(&line) {
                c.get(1).unwrap().as_str().to_string()
            } else {
                return Err(Error::ReqwestError("No video details".to_string()));
            }
        };

        macro_rules! extract_detail {
            ($reg:expr, $opt:expr) => {{
                let regex = Regex::new($reg).unwrap();
                if let Some(c) = regex.captures(&video_details) {
                    c.get(1).unwrap().as_str()
                } else {
                    if $opt {
                        ""
                    } else {
                        return Err(Error::ReqwestError(format!("{} failed", $reg)));
                    }
                }
            }};
        }
        let title = extract_detail!(r#""title":"(.+?)",""#, false).decode_html();
        let length_seconds = extract_detail!(r#""lengthSeconds":"(.+?)""#, false).parse::<u32>()?;
        let keywords = extract_detail!(r#""keywords":\[(.+?)\]"#, true)
            .split(',')
            .map(|x| x.decode_html().replace('"', ""))
            .collect();
        let channel_id = extract_detail!(r#""channelId":"(.+?)""#, false).decode_html();
        let description = extract_detail!(r#""shortDescription":"(.+?)""#, false).decode_html();
        let views = extract_detail!(r#""viewCount":"(.+?)""#, false).parse::<u32>()?;
        let author = extract_detail!(r#""author":"(.+?)""#, false).decode_html();

        Ok(Video {
            title,
            channel_id,
            id,
            keywords,
            length_seconds,
            description,
            url,
            views,
            author,
            status: VideoStatus::Unseen,
            source: VideoService::Youtube,
        })
    } else {
        Err(Error::ReqwestError(format!(
            "Network request failed: {}",
            res.status()
        )))
    }
}

pub fn get_video_url(id: &str) -> String {
    format!("https://www.youtube.com/watch?v={}", id)
}
