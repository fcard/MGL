use crate::utility::try_map::*;

fn ok<T>(x: T) -> Result<T, ()> {
  Ok(x)
}

fn err<T>(_: T) -> Result<T, ()> {
  Err(())
}


#[test]
fn test_utility_try_map() {
  assert_eq!(
    vec![1,2,3,4].try_map(ok),
    Ok(vec![1,2,3,4])
  )
}

#[test]
fn test_utility_try_map_collect() {
  assert_eq!(
    vec![1,2,3,4].try_map_collect(ok),
    Ok(vec![1,2,3,4])
  )
}

#[test]
fn test_utility_try_map_errors() {
  assert_eq!(
    vec![1,2,3,4].try_map(err),
    Err(vec![(),(),(),()])
  )
}

