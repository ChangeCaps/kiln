use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    path::PathBuf,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ShaderIncludePath {
    Builtin(String),
    Path(PathBuf),
}

#[derive(Clone)]
pub struct ShaderInclude {
    pub includes: HashSet<ShaderIncludePath>,
    pub source: Cow<'static, str>,
}

#[derive(Clone)]
pub struct ShaderProcessor {
    includes: HashMap<ShaderIncludePath, ShaderInclude>,
}
