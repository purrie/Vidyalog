use htmlentity::entity::decode;

use super::HTMLDecodable;

impl HTMLDecodable for &str {
    fn decode_html(&self) -> String {
        decode_html(self)
    }
}

fn decode_html(text: &str) -> String {
    decode(text)
        .iter()
        .fold(String::new(), |t, n| format!("{}{}", t, n).replace(r#"\""#, r#"""#))
}
