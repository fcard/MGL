use std::iter::FromIterator;


pub trait ResultFn<T,E> = Fn(T) -> Result<T, E>;

pub trait TryMap<Item>: Sized {

  fn try_map_collect<E, F, I>(self, f: F) -> Result<I, Vec<E>>
    where F: ResultFn<Item, E>,
          I: FromIterator<Item> {

    self.try_map(f).map(|v| v.into_iter().collect())
  }


  fn try_map<E, F>(self, f: F) -> Result<Vec<Item>, Vec<E>>
    where F: ResultFn<Item, E>;
}


impl<Item, Into: Iterator<Item=Item>, I> TryMap<Item> for I
  where I: IntoIterator<Item=Item, IntoIter=Into> {

  fn try_map<E, F>(self, f: F) -> Result<Vec<Item>, Vec<E>>
    where F: ResultFn<Item, E> {

    let mut result = Vec::new();
    let mut errors = Vec::new();
    for item in self {
      match f(item) {
        Ok(r)  => { result.push(r) }
        Err(e) => { errors.push(e) }
      }
    }
    if errors.is_empty() {
      Ok(result)
    } else {
      Err(errors)
    }
  }
}

