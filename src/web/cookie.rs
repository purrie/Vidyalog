use std::collections::HashMap;

use reqwest::header::HeaderValue;

use super::Cookie;

impl Cookie {
    /// Parses the header value to create a cookie
    pub fn parse(cookie_raw: &HeaderValue) -> Cookie {
        let mut cookies = HashMap::new();
        let mut cookie = Cookie::default();
        cookie_raw
            .to_str()
            .unwrap()
            .split(';')
            .enumerate()
            .for_each(|x| {
                let i = x.0;
                let x = x.1;
                if x.contains('=') {
                    let mut pair: Vec<String> = x.split('=').map(|x| x.to_string()).collect();
                    let val = pair.pop().unwrap();
                    let key = pair.pop().unwrap();
                    if i == 0 {
                        cookie.key = key;
                        cookie.value = val;
                    } else {
                        cookies.insert(key, val);
                    }
                } else {
                    cookies.insert(x.to_string(), String::new());
                }
            });
        for x in cookies {
            match x.0.to_lowercase().as_str() {
                "expires" => cookie.expires = x.1,
                "path" => cookie.path = x.1,
                "domain" => cookie.domain = x.1,
                _ if x.1.len() > 0 => drop(cookie.maps.insert(x.0, x.1)),
                _ => cookie.tags.push(x.0),
            }
        }
        cookie
    }
}
