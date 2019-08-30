use std::io::Write;
use std::borrow::Cow;
use crate::error::*;
use quick_xml::{Writer, events::{Event, BytesEnd, BytesStart, BytesText, attributes::Attribute}};

pub trait ResourceToXml {
  fn write_xml<W: Write>(&self, w: W) -> Result<W>;
}

pub fn write_events<'a, I, W: Write>(w: W, events: I) -> Result<W>
  where I: IntoIterator<Item=Event<'a>> {

  let mut writer = Writer::new(w);
  for event in events {
    writer.write_event(event)?;
  }
  Ok(writer.into_inner())
}


pub trait Attributes<'a, T> = IntoIterator<Item=T> where T: Into<Attribute<'a>>;

pub fn start<'a, T, I: Attributes<'a, T>>(name: &str, attributes: I) -> Event<'static> {
  Event::Start(bytes_start(name, attributes))
}

pub fn empty<'a, T, I: Attributes<'a, T>>(name: &str, attributes: I) -> Event<'static> {
  Event::Empty(bytes_start(name, attributes))
}

pub fn bytes_start<'a, T, I: Attributes<'a, T>>(name: &str, attributes: I) -> BytesStart<'static> {
  let mut element = BytesStart::owned_name(name.as_bytes().to_vec());
  element.extend_attributes(attributes);
  element
}




pub fn end(name: &str) -> Event<'static> {
  Event::End(BytesEnd::owned(name.as_bytes().to_vec()))
}

pub fn text<'a>(content: &'a str) -> Event<'static> {
  Event::Text(BytesText::from_plain_str(content).into_owned())
}

pub fn attr<'a, T, U>(key: &'a T, value: &'a U) -> Attribute<'a>
  where T: AsRef<str>,
        U: AsRef<str> {

  Attribute {
    key:   key.as_ref().as_bytes(),
    value: Cow::from(value.as_ref().as_bytes())
  }
}


pub macro start($tag: expr$(, $($attrs: tt)*)?)  {
  attrs!(start, $tag $(,$($attrs)*)*)
}

pub macro empty($tag: expr$(, $($attrs: tt)*)?)  {
  attrs!(empty, $tag $(,$($attrs)*)*)
}

pub macro attrs($caller: ident, $tag: expr $(,$key: expr => $value: expr)*) {{
  let v: Vec<(String, String)> =
    vec![
      $(($key.to_string(), $value.to_string())),*
    ];
  $caller($tag, v.iter().map(|(k,v)| attr(k, v)))
}}

