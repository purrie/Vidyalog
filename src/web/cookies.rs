use reqwest::{header::HeaderMap, RequestBuilder};

use super::{Cookie, Cookies};

impl Cookies {
    /// Adds cookies to the request headers
    /// TODO limit the cookies to their domain
    pub fn add_to_headers(&self, request: RequestBuilder) -> RequestBuilder {
        let cookies = self.cookies.iter().enumerate().fold(String::new(), |f, x| {
            let i = x.0;
            let x = x.1;
            if i == 0 {
                format!("{}={}", x.key, x.value)
            } else {
                format!("{}; {}={}", f, x.key, x.value)
            }
        });

        request.header("cookie", cookies)
    }
    /// Updates and adds cookies from response header
    pub fn update(&mut self, response: &HeaderMap) {
        response
            .iter()
            .filter(|x| x.0.as_str().to_lowercase() == "set-cookie")
            .map(|x| Cookie::parse(x.1))
            .for_each(|x| {
                if let Some(p) = self
                    .cookies
                    .iter()
                    .position(|c| c.domain == x.domain && c.key == x.key)
                {
                    self.cookies.remove(p);
                }
                self.cookies.push(x);
            });
    }
}
