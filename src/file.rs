use std::{
    fs::{create_dir_all, read, read_dir, remove_file, write},
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};

use crate::{enums::Error, service::ContentID};

/// Name of the folder in which data should be kept in expected file system paths
pub const PROJECT_NAME: &str = "vidyalog";

/// This trait automatically implements functions for serializing and deserializing data.
///
/// It is expected that a struct implements either a ContentID or FileID traits that guide what functions are available.
/// The implementing type also must choose a PathProvider that dictates where the files are meant to be stored.
pub trait File {
    type Path: PathProvider;

    /// Serializes the data of this object to drive according to the PathProvider and ContentID of the object.
    fn save_content(&self) -> Result<(), Error>
    where
        Self: Serialize + ContentID + Sized,
    {
        let mut path = self.get_content_drive_path();
        if path.exists() == false {
            create_dir_all(&path)?;
        }
        let id = self.get_content_id();
        path.push(id.id);
        path.set_extension("ron");

        let pretty = ron::ser::PrettyConfig::default();
        let ser = ron::ser::to_string_pretty(self, pretty)?;
        write(path, ser.as_bytes())?;
        Ok(())
    }
    /// Serializes data of the object to the drive. This is a SingleFileID implementation, meant for singleton-type objects as there is only one path per struct type
    fn save_file(&self) -> Result<(), Error>
    where
        Self: Serialize + Sized + SingleFileID,
    {
        let name = Self::get_file_id();
        let mut path = Self::Path::path();
        if path.exists() == false {
            create_dir_all(&path)?;
        }
        path.push(name);
        path.set_extension("ron");

        let pretty = ron::ser::PrettyConfig::default();
        let ser = ron::ser::to_string_pretty(self, pretty)?;
        write(path, ser.as_bytes())?;
        Ok(())
    }
    /// Deserializes data from the drive and creates an instance of the object.
    ///
    /// It is meant for use in SingleFileID structs that work in singleton-type way as only one path is available per struct type
    fn load_file() -> Result<Self, Error>
    where
        Self: DeserializeOwned + SingleFileID,
    {
        let mut path = Self::Path::path();
        let name = Self::get_file_id();
        path.push(name);
        path.set_extension("ron");

        let buffer = read(path)?;
        let r = ron::de::from_bytes::<Self>(&buffer);
        match r {
            Ok(o) => Ok(o),
            Err(e) => Err(e.into()),
        }
    }
    /// A general function that loads data from provided path to create an instance of the struct.
    ///
    /// The function accepts both absolute and relative paths.
    ///
    /// # Warning
    /// This function isn't meant to be used directly, rather, it is meant for being called by the load_all() function.
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
    /// This function loads all data from expected path as dictated by PathProvider.
    ///
    /// # Errors
    /// This function will return empty Vec if the path doesn't exist.
    ///
    /// Otherwise, if any of the files in the path don't load in correctly, the function will continue loading other files and the failed files will be omited.
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
    /// This function deletes the data of the file from the drive as well as drops the struct it's being called on.
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
    /// Convenience function for retrieving path in which data of this object are stored
    fn get_content_drive_path(&self) -> PathBuf
    where
        Self: ContentID + Sized,
    {
        let mut p = Self::Path::path();
        let id = self.get_content_id();
        p.push(id.service.get_path_name());
        p
    }
    /// Convenience function for retrieving path to this file, by default, the extension is set to "ron"
    fn get_content_file_path(&self) -> PathBuf
    where
        Self: ContentID + Sized,
    {
        let mut p = self.get_content_drive_path();
        let id = self.get_content_id();
        p.push(id.id);
        p.set_extension("ron");
        p
    }
}

/// PathProvider is used for conveniently declaring where the data associated with the file type should be stored.
pub trait PathProvider {
    fn path() -> PathBuf;
}
/// This trait is used to declare the struct as serializable as a singleton-type struct.
/// Meaning, there's only one file for the type and it is expected that no more than one instance of the struct exists.
pub trait SingleFileID {
    fn get_file_id() -> &'static str;
}

/// Provides a path for storing playlist data
pub struct PlaylistPath;
/// Provides a path for storing video data
pub struct VideoPath;
/// Provides a path for storing a thumbnail data
pub struct ThumbnailPath;
/// Provides a path for storing program settings
pub struct SettingsPath;

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
impl PathProvider for ThumbnailPath {
    fn path() -> PathBuf {
        let mut d = dirs::data_dir().unwrap();
        d.push(PROJECT_NAME);
        d.push("thumbnails");
        d
    }
}
impl PathProvider for SettingsPath {
    fn path() -> PathBuf {
        let mut d = dirs::config_dir().unwrap();
        d.push(PROJECT_NAME);
        d
    }
}
