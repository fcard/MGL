//! Holds the Abstract Syntax Tree types, as well as
//! convenience functions and trait implementations,

pub mod wrapper;
pub mod expressions;
pub mod operators;
pub mod statements;
pub mod declarations;
pub mod precedence;
pub mod std_traits;

pub use wrapper::*;
pub use expressions::*;
pub use operators::*;
pub use statements::*;
pub use declarations::*;

#[derive(Debug, Clone)]
pub struct Top {
  pub declarations: Vec<Declaration>
}

impl Top {
  pub fn new(d: &[Declaration]) -> Self {
    Top {
      declarations: Vec::from(d)
    }
  }
}

