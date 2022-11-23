use reqwest::Response;

use crate::enums::Error;

use super::Browser;

impl Browser {
    pub async fn open(&mut self, url: &str) -> Result<Response, Error> {
        let req = self
            .client
            .get(url)
            .header("user-agent", &self.user_agent)
            .header("accept", &self.accept)
            .header("accept-language", &self.accept_language);

        let req = self.cookies.add_to_headers(req);

        let res = req.send().await?;
        self.cookies.update(res.headers());
        Ok(res)
    }
}

impl Default for Browser {
    fn default() -> Self {
        Self {
            cookies: Default::default(),
            client: Default::default(),
            accept: "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8".to_string(),
            accept_language: "en-US,en;q=0.7".to_string(),
            user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36".to_string(),
        }
    }
}
