use crate::parser::ast::*;

#[derive(Debug, Clone)]
pub struct Script {
  source: FunctionDeclaration,
  kind: ScriptKind,
}

#[derive(Debug, Clone)]
pub enum ScriptKind {
  Global,
  Method(ResourceName),
}

