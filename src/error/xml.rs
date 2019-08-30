use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, self};
use std::sync::Mutex;
use lazy_static::*;
use quick_xml;
use crate::error::*;

impl From<quick_xml::Error> for MglError {
  fn from(err: quick_xml::Error) -> Self {
    MglError::Xml(XmlError::new(err))
  }
}



#[derive(Clone, Eq)]
pub struct XmlError(usize);


lazy_static! {
  static ref XML_ERROR_ID: Mutex<usize> = Mutex::new(0);

  static ref XML_ERRORS: Mutex<HashMap<usize, quick_xml::Error>> = {
    Mutex::new(HashMap::new())
  };
}

macro xml_errors() {
  XML_ERRORS.try_lock().expect("XML_ERRORS is locked!")
}

macro xml_error_id() {
  XML_ERROR_ID.try_lock().expect("XML_ERROR_ID is locked!")
}



impl XmlError {
  pub fn new(err: quick_xml::Error) -> Self {
    let id = *xml_error_id!();
    *xml_error_id!() += 1;

    xml_errors!().insert(id, err);
    XmlError(id)
  }

  fn with_err_mut<R, F: FnMut(&mut quick_xml::Error) -> R>(&self, mut func: F) -> R {
    func(&mut xml_errors!().get_mut(&self.0).unwrap())
  }
}

impl PartialEq for XmlError {
  fn eq(&self, other: &XmlError) -> bool {
    format!("{:?}", self) == format!("{:?}", other)
  }
}

impl From<XmlError> for quick_xml::Error {
  fn from(err: XmlError) -> Self {
    xml_errors!().remove(&err.0).unwrap()
  }
}

impl Drop for XmlError {
  fn drop(&mut self) {
    xml_errors!().remove(&self.0);
  }
}

impl Debug for XmlError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    self.with_err_mut(|err| {
      write!(f, "{:?}", err)
    })
  }
}

impl Display for XmlError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    self.with_err_mut(|err| {
      write!(f, "{}", err)
    })
  }
}

