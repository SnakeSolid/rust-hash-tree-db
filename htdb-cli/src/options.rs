use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "hash-tree-db")]
pub struct Options {
    #[structopt(short, long, default_value = "8192")]
    page_size: usize,

    #[structopt(short, long)]
    memory_pages: Option<usize>,

    #[structopt(short, long, default_value = ".", parse(from_os_str))]
    storage_path: PathBuf,
}

impl Options {
    pub fn page_size(&self) -> usize {
        self.page_size
    }

    pub fn memory_pages(&self) -> Option<usize> {
        self.memory_pages
    }

    pub fn storage_path(&self) -> &Path {
        self.storage_path.as_path()
    }
}
