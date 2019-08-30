use quick_xml;
use crate::error::xml::*;

#[test]
fn test_compiler_gmx_xml_error() {
  let err1 = XmlError::new(quick_xml::Error::TextNotFound);
  let err2 = XmlError::new(quick_xml::Error::TextNotFound);
  let err3 = XmlError::new(quick_xml::Error::UnexpectedBang);

  assert_eq!(err1, err2);
  assert_ne!(err1, err3);

  assert_eq!(format!("{:?}", err1), format!("{:?}", quick_xml::Error::TextNotFound));
  assert_eq!(format!("{}",   err1), format!("{}",   quick_xml::Error::TextNotFound));
}

