use quick_xml;
use std::io::{Write, self};
use std::fmt;
use std::error::Error;
use crate::tests::utility::*;
use crate::error::*;
use crate::error::xml::*;
use crate::resources::room::*;
use crate::resources::instance::*;
use crate::resources::resource_trait::*;
use crate::compiler::gmx::to_xml_trait::ResourceToXml;

#[derive(Debug, PartialEq)]
struct InvalidWriter;

#[derive(Debug, PartialEq)]
struct WhateverError;

impl io::Write for InvalidWriter {
  fn write(&mut self, _: &[u8]) -> io::Result<usize> {
    Err(io::Error::new(io::ErrorKind::Other, box WhateverError))
  }

  fn flush(&mut self) -> io::Result<()> {
    Ok(())
  }
}

impl fmt::Display for WhateverError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for WhateverError {}


#[test]
fn test_compiler_gmx_xml_error() {
  let err1 = XmlError::new(quick_xml::Error::TextNotFound);
  let err2 = XmlError::new(quick_xml::Error::TextNotFound);
  let err3 = XmlError::new(quick_xml::Error::UnexpectedBang);

  assert_eq!(err1, err2);
  assert_ne!(err1, err3);

  assert_eq!(err1, err1.clone());

  assert_eq!(format!("{:?}", err1), format!("{:?}", quick_xml::Error::TextNotFound));
  assert_eq!(format!("{}",   err1), format!("{}",   quick_xml::Error::TextNotFound));
}

#[test]
fn test_compiler_gmx_mgl_xml_error() {
  let room = Room::new((resource("room r {}"), vec![])).unwrap();
  let result = room.write_xml(InvalidWriter);

  assert_eq!(
    result,
    Err(
      MglError::Xml(
        XmlError::new(
          quick_xml::Error::Io(
            InvalidWriter.write(&[]).unwrap_err()
          )
        )
      )
    )
  )
}

#[test]
fn test_compiler_gmx_room() {
  let mut buffer = Vec::new();

  let instance = Instance::new(instance("
    instance i of o {
      x: 1
      y: 2
    }
  ")).unwrap();


  let room = Room::new((resource("
    room r {
      persistent: true
    }
  "), vec![
    InstanceItem::Resolved(rn!(i), instance.clone()),
    InstanceItem::Resolved(rn!(instance::i), instance)
  ])).unwrap();


  println!("{:?}", room.instances);

  room.write_xml(&mut buffer);
}
