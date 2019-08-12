pub mod expressions;
pub mod operators;
pub mod statements;
pub mod declarations;
pub mod precedence;

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

