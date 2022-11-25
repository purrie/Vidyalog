use std::{
    fs::{create_dir_all, read, read_dir, remove_file, write},
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};

use crate::{enums::Error, service::ContentID};

pub const PROJECT_NAME: &str = "vidyalog";

pub trait File {
    type Path: PathProvider;

    fn save(&self) -> Result<(), Error>
    where
        Self: Serialize + ContentID + Sized,
    {
        let mut path = Self::Path::path();
        let id = self.get_content_id();
        path.push(id.service.get_path_name());
        if path.exists() == false {
            create_dir_all(&path)?;
        }
        path.push(id.id);
        path.set_extension("ron");

        let pretty = ron::ser::PrettyConfig::default();
        let ser = ron::ser::to_string_pretty(self, pretty)?;
        write(path, ser.as_bytes())?;
        Ok(())
    }
    fn load_path<P>(path: P) -> Result<Self, Error>
    where
        Self: DeserializeOwned,
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let buffer = read(path)?;
        let r = ron::de::from_bytes::<Self>(&buffer);
        match r {
            Ok(o) => Ok(o),
            Err(e) => Err(e.into()),
        }
    }
    fn load_all() -> Vec<Self>
    where
        Self: DeserializeOwned,
    {
        let path = Self::Path::path();
        if path.exists() == false {
            return Vec::new();
        }
        let mut vec = Vec::new();
        let mut dirs = Vec::new();
        dirs.push(path);
        while let Some(path) = dirs.pop() {
            if let Ok(files) = read_dir(path) {
                for file in files {
                    let Ok(file) = file else {
                        continue
                    };
                    let Ok(typ) = file.file_type() else {
                        continue
                    };
                    let p = file.path();
                    if typ.is_dir() {
                        dirs.push(p);
                    } else {
                        let Ok(r) = Self::load_path(p) else {
                            continue
                        };
                        vec.push(r);
                    }
                }
            }
        }
        vec
    }
    fn delete(self) -> Result<(), Error>
    where
        Self: Sized + ContentID,
    {
        let mut path = Self::Path::path();
        let id = self.get_content_id();
        path.push(id.service.get_path_name());
        path.push(id.id);
        path.set_extension("ron");
        remove_file(path)?;
        Ok(())
    }
}

pub trait PathProvider {
    fn path() -> PathBuf;
}

pub struct PlaylistPath();
pub struct VideoPath();

impl PathProvider for PlaylistPath {
    fn path() -> PathBuf {
        let mut d = dirs::data_dir().unwrap();
        d.push(PROJECT_NAME);
        d.push("playlists");
        d
    }
}
impl PathProvider for VideoPath {
    fn path() -> PathBuf {
        let mut d = dirs::data_dir().unwrap();
        d.push(PROJECT_NAME);
        d.push("videos");
        d
    }
}
