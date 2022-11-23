pub const PROJECT_NAME: &str = "vidyalog";

macro_rules! playlists_path {
    () => {{
        let mut d = dirs::data_dir().unwrap();
        d.push(crate::paths::PROJECT_NAME);
        d.push("playlists");
        d
    }};
}

pub(crate) use playlists_path;
