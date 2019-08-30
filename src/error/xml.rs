use std::result::Result as StdResult;

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, self};
use std::sync::{Mutex, MutexGuard, PoisonError};
use lazy_static::*;
use quick_xml;
use crate::error::*;

impl From<quick_xml::Error> for MglError {
  fn from(err: quick_xml::Error) -> Self {
    MglError::Xml(XmlError::new(err))
  }
}



#[derive(Eq)]
pub struct XmlError(usize);

struct XmlErrorData {
  err: quick_xml::Error,
  clones: usize,
}

impl XmlErrorData {
  fn new(err: quick_xml::Error) -> Self {
    XmlErrorData {
      err,
      clones: 0
    }
  }
}

struct XmlErrorHolder {
  id: Mutex<usize>,
  errs: UnsafeCell<HashMap<usize, XmlErrorData>>,
  mutex: Mutex<()>
}

impl XmlErrorHolder {
  fn new() -> Self {
    XmlErrorHolder {
      id:   Mutex::new(0),
      errs: UnsafeCell::new(HashMap::new()),
      mutex: Mutex::new(())
    }
  }

  fn data<'a>(&'a self, i: usize) -> Option<&'a XmlErrorData> {
    unsafe {
      (*self.errs.get()).get(&i)
    }
  }

  fn err<'a>(&'a self, i: usize) -> Option<&'a quick_xml::Error> {
    self.data(i).map(|d| &d.err)
  }

  fn clones<'a>(&'a self, i: usize) -> Option<usize> {
    self.data(i).map(|d| d.clones)
  }

  fn lock(&self) -> StdResult<MutexGuard<()>, PoisonError<MutexGuard<()>>> {
    self.mutex.lock()
  }

  fn push(&self, err: quick_xml::Error) -> usize {
    let mut id = self.id.lock().unwrap();
    unsafe {
      (*self.errs.get()).insert(*id, XmlErrorData::new(err));
    }

    *id += 1;
    return *id - 1;
  }

  fn set_clones(&self, i: usize, c: usize) {
    let _lock = self.lock();
    unsafe {
      (*self.errs.get()).get_mut(&i).map(|d| d.clones = c);
    }
  }

  fn remove(&self, i: usize) {
    let _lock = self.lock();
    unsafe {
      (*self.errs.get()).remove(&i);
    }
  }
}

unsafe impl Sync for XmlErrorHolder {}

lazy_static! {
  static ref XML_ERRORS: XmlErrorHolder = XmlErrorHolder::new();
}


impl XmlError {
  pub fn new(err: quick_xml::Error) -> Self {
    let id = XML_ERRORS.push(err);
    XmlError(id)
  }

  fn clones(&self) -> usize {
    XML_ERRORS.clones(self.0).unwrap()
  }

  fn err<'a>(&'a self) -> &'a quick_xml::Error {
    XML_ERRORS.err(self.0).unwrap()
  }
}

impl PartialEq for XmlError {
  fn eq(&self, other: &XmlError) -> bool {
    format!("{:?}", self) == format!("{:?}", other)
  }
}

impl AsRef<quick_xml::Error> for XmlError {
  fn as_ref(&self) -> &quick_xml::Error {
    self.err()
  }
}

impl Clone for XmlError {
  fn clone(&self) -> Self {
    XML_ERRORS.set_clones(self.0, self.clones() + 1);
    XmlError(self.0)
  }
}

impl Drop for XmlError {
  fn drop(&mut self) {
    let clones = self.clones();

    if clones > 0 {
      XML_ERRORS.set_clones(self.0, clones-1);

    } else {
      XML_ERRORS.remove(self.0);
    }
  }
}

impl Debug for XmlError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self.err())
  }
}

impl Display for XmlError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.err())
  }
}

