use std::{
    fs::{create_dir_all, read, read_dir, remove_file, write},
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};

use crate::{enums::Error, paths::playlists_path};

pub trait File {
    type Path: PathProvider;

    fn save(&self) -> Result<(), Error>
    where
        Self: Serialize + FileID,
    {
        let mut path = Self::Path::path();
        if path.exists() == false {
            create_dir_all(&path)?;
        }
        path.push(&self.get_file_id());
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
    fn load_id(id: &str) -> Result<Self, Error>
    where
        Self: DeserializeOwned + FileID,
    {
        let mut path = Self::Path::path();
        path.push(id);
        path.set_extension("ron");
        let buff = read(path)?;
        let r = ron::de::from_bytes::<Self>(&buff);
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
        if let Ok(files) = read_dir(path) {
            for file in files {
                let p = match file {
                    Ok(f) => f.path(),
                    Err(_) => continue,
                };
                let r = match Self::load_path(p) {
                    Ok(r) => r,
                    Err(_) => continue,
                };
                vec.push(r);
            }
        }
        vec
    }
    fn delete(self) -> Result<(), Error>
    where
        Self: Sized + FileID,
    {
        let mut path = Self::Path::path();
        path.push(self.get_file_id());
        path.set_extension("ron");
        remove_file(path)?;
        Ok(())
    }
}

pub trait PathProvider {
    fn path() -> PathBuf;
}
pub trait FileID {
    fn get_file_id(&self) -> &str;
}

pub struct PlaylistPath();
impl PathProvider for PlaylistPath {
    fn path() -> PathBuf {
        playlists_path!()
    }
}
