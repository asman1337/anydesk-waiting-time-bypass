use std::path::Path;
use std::{fs, io};

use logs::{error, info, warn};

pub struct DirectoryCleaner;

#[allow(dead_code)]
impl DirectoryCleaner {
    /// Checks if a directory exists.
    fn directory_exists(path: &Path) -> bool {
        path.is_dir()
    }
    /// Deletes a directory if it exists.
    fn delete_directory(path: &Path) -> io::Result<()> {
        if Self::directory_exists(path) {
            info!("Deleting directory: {:?}", path);
            match fs::remove_dir_all(path) {
                Ok(_) => info!("Successfully deleted directory: {:?}", path),
                Err(e) => error!("Failed to delete directory: {:?}, Error: {}", path, e),
            }
        } else {
            warn!("Directory does not exist: {:?}", path);
        }
        Ok(())
    }

    /// Deletes a list of directories.
    pub fn delete_directories(paths: &[&str]) {
        for path_str in paths {
            let path = Path::new(path_str);
            if let Err(e) = Self::delete_directory(path) {
                error!("Error deleting directory {:?}: {}", path, e);
            }
        }
    }

    /// Get the list of directories to clean based on the OS.
    pub fn get_directories_to_clean() -> Vec<&'static str> {
        todo!("Will implement later ...")
    }
}
