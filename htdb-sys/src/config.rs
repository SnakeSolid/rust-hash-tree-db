use std::path::{Path, PathBuf};

const MAX_PAGE_SIZE: usize = 128;
const MAX_PAGES: Option<usize> = None;

#[derive(Debug)]
pub struct Config {
    max_page_size: usize,
    max_pages: Option<usize>,
    storage_path: PathBuf,
}

impl Config {
    pub fn set_max_page_size(mut self, max_page_size: usize) -> Self {
        self.max_page_size = max_page_size;
        self
    }

    pub fn max_page_size(&self) -> usize {
        self.max_page_size
    }

    pub fn set_max_pages(mut self, max_pages: Option<usize>) -> Self {
        self.max_pages = max_pages;
        self
    }

    pub fn max_pages(&self) -> Option<usize> {
        self.max_pages
    }

    pub fn set_storage_path<P>(mut self, storage_path: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.storage_path = storage_path.as_ref().to_path_buf();
        self
    }

    pub fn storage_path(&self) -> &Path {
        self.storage_path.as_path()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_page_size: MAX_PAGE_SIZE,
            max_pages: MAX_PAGES,
            storage_path: PathBuf::from("."),
        }
    }
}
