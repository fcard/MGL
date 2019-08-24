use std::fmt;
use std::sync::Mutex;
use std::path::PathBuf;
use lazy_static::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SourceFile {
  None,
  File(usize)
}

lazy_static! {
  static ref SOURCE_FILES: Mutex<Vec<PathBuf>> = Mutex::new(vec![]);
}

macro source_files() {
  SOURCE_FILES.lock().unwrap()
}

impl SourceFile {
  pub fn new(p: PathBuf) -> Self {
    let found = source_files!().iter().position(|p2| p == *p2);

    if let Some(index) = found {
      SourceFile::File(index)
    } else {
      source_files!().push(p);
      SourceFile::File(source_files!().len() - 1)
    }
  }

  pub fn as_path(self) -> Option<PathBuf> {
    match self {
      SourceFile::None => None,
      SourceFile::File(index) => {
        source_files!().get(index).map(PathBuf::clone)
      }
    }
  }
}

impl fmt::Debug for SourceFile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      SourceFile::None => {
        write!(f, "None")
      }

      SourceFile::File(index) => {
        write!(f, "File({:?}={:?})", index, source_files!()[index])
      }
    }
  }
}
