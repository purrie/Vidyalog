use htmlentity::entity::decode;

use super::HTMLDecodable;

impl HTMLDecodable for &str {
    /// Decodes html escape strings into actual characters
    fn decode_html(&self) -> String {
        decode_html(self)
    }
}

/// Decodes html escape character strings into actual characters
fn decode_html(text: &str) -> String {
    decode(text)
        .iter()
        .fold(String::new(), |t, n| format!("{}{}", t, n).replace(r#"\""#, r#"""#))
}
