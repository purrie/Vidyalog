use iced::widget::image;
use regex::{Captures, Regex};

use crate::{
    data::{Playlist, Thumbnail, Video},
    enums::{ContentType, Error, VideoService, VideoStatus},
    web::HTMLDecodable,
};

use super::{BrowserCarrier, ContentIdentifier};

/// Visits the url to grab information about the playlist
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
    let url = get_playlist_url(&id);
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

        let videos: Vec<_> = {
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
                .iter()
                .map(|x| ContentIdentifier::new(&VideoService::Youtube, x, ContentType::Video))
                .collect()
        };
        let description = {
            let regex = Regex::new(r#""descriptionText":\{"simpleText":"(.+?)"\}"#).unwrap();
            if let Some(c) = regex.captures(&playlist_data) {
                c.get(1).unwrap().as_str().decode_html()
            } else {
                String::new()
            }
        };
        let author = {
            let regex = Regex::new(r#""ownerText":\{"runs":\[\{"text":"(.+?)",""#).unwrap();
            if let Some(c) = regex.captures(&playlist_data) {
                c.get(1).unwrap().as_str().decode_html()
            } else {
                String::from("Unknown Author")
            }
        };
        let thumbnail = {
            let regex = Regex::new(r#"heroPlaylistThumbnailRenderer":\{"thumbnail":\{"thumbnails":\[\{"url":"https://i.ytimg.com/vi/(.+?)/hqdefault.jpg"#).unwrap();
            if let Some(c) = regex.captures(&playlist_data) {
                let id = c.get(1).unwrap().as_str().to_string();
                ContentIdentifier::new(&VideoService::Youtube, &id, ContentType::Thumbnail)
            } else {
                let vid = videos.get(0).unwrap();
                ContentIdentifier::new(&VideoService::Youtube, &vid.id, ContentType::Thumbnail)
            }
        };
        Ok(Playlist {
            id,
            url,
            title,
            videos,
            description,
            author,
            thumbnail,
            source: VideoService::Youtube,
            tracked: false,
        })
    } else {
        Err(Error::ReqwestError(format!(
            "Network request failed: {}",
            res.status()
        )))
    }
}

/// Visits the url to grab the information about the video
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
            thumbnail: ContentIdentifier::new(&VideoService::Youtube, &id, ContentType::Thumbnail),
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

/// Visits the url to grab information about the thumbnail, it will also download the jpg thumbnail file
pub async fn procure_thumbnail(browser: BrowserCarrier, url: String) -> Result<Thumbnail, Error> {
    let mut browser = browser.lock().await;

    let id = {
        let regex = Regex::new(r"https://i.ytimg.com/vi/(.+?)/").unwrap();
        if let Some(c) = regex.captures(&url) {
            c.get(1).unwrap().as_str().to_string()
        } else {
            return Err(Error::InvalidThumbnailURL(url));
        }
    };

    let res = browser.open(&url).await?;

    if res.status() == 200 {
        let bytes = res.bytes().await?;
        let img = image::Handle::from_memory(bytes.to_vec());

        let th = Thumbnail::new(url, id, VideoService::Youtube).with_image(img);
        Ok(th)
    } else {
        Err(Error::ReqwestError(format!(
            "Network request failed: {}",
            res.status()
        )))
    }
}

/// Converts the id into a playlist url
///
/// There is no validation as to whatever the returned url is actually a playlist url, the function just formats the string
pub fn get_playlist_url(id: &str) -> String {
    format!("https://www.youtube.com/playlist?list={}", id)
}
/// Converts the id into a video url
///
/// There is no validation as to whatever the returned url is actually a video url, the function just formats the string
pub fn get_video_url(id: &str) -> String {
    format!("https://www.youtube.com/watch?v={}", id)
}
/// Converts the id into a thumbnail url pointing to the image file
///
/// There is no validation as to whatever the returned url is actually a thumbnail url, the function just formats the string
pub fn get_thumbnail_url(id: &str) -> String {
    format!("https://i.ytimg.com/vi/{}/hqdefault.jpg", id)
}
