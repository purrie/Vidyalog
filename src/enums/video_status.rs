use super::VideoStatus;

impl From<u32> for VideoStatus {
    fn from(n: u32) -> Self {
        if n == 0 {
            Self::Unseen
        } else {
            Self::Seen(n)
        }
    }
}

