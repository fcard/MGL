use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Script {
  source: FunctionDeclaration,
  kind: ScriptKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScriptKind {
  Global,
  Method(ResourceName),
}

impl Script {
  pub fn global(source: FunctionDeclaration) -> Script {
    Script {
      source,
      kind: ScriptKind::Global
    }
  }

  pub fn method(source: FunctionDeclaration, name: ResourceName) -> Script {
    Script {
      source,
      kind: ScriptKind::Method(name)
    }
  }
}

