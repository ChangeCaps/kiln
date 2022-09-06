use std::{
    borrow::Cow,
    collections::{HashMap, HashSet, VecDeque},
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

use thiserror::Error;

use crate::error::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ShaderIncludePath<'a> {
    Global(String),
    Local(Cow<'a, Path>),
}

impl<'a> ShaderIncludePath<'a> {
    fn char_allowed(ch: char) -> bool {
        const BANNED: &[char] = &[0 as char, '\\', ':', '*', '?', '"', '<', '>', '|'];
        !BANNED.contains(&ch)
    }

    fn verify_path(path: &str) -> Result<(), ShaderProcessorError> {
        if let Some(ch) = path.chars().find(|&ch| !Self::char_allowed(ch)) {
            Err(ShaderProcessorError::BadChar(ch))
        } else {
            Ok(())
        }
    }

    pub fn is_local(&self) -> bool {
        match self {
            ShaderIncludePath::Global(_) => false,
            ShaderIncludePath::Local(_) => true,
        }
    }

    pub fn parse(source: &str, path: Option<&Path>) -> Result<Self, ShaderProcessorError> {
        match source.chars().next() {
            Some('"') => {
                let path_source = source.strip_prefix('"').unwrap().strip_suffix('"');

                match path_source {
                    Some(path_source) => {
                        Self::verify_path(path_source)?;

                        let full_path = path
                            .expect("path not supplied for include path {internal error}")
                            .parent()
                            .expect("all files should have a parent")
                            .join(Path::new(path_source));
                        Ok(Self::Local(full_path.into()))
                    }
                    None => Err(ShaderProcessorError::BadEnd('"')),
                }
            }
            Some('<') => {
                let path_source = source.strip_prefix('<').unwrap().strip_suffix('>');

                match path_source {
                    Some(path_source) => {
                        Self::verify_path(path_source)?;
                        Ok(Self::Global(path_source.into()))
                    }
                    None => Err(ShaderProcessorError::BadEnd('>')),
                }
            }
            Some(ch) => Err(ShaderProcessorError::BadStart(ch)),
            None => Err(ShaderProcessorError::ExpectedPath),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShaderInclude {
    pub includes: HashSet<ShaderIncludePath<'static>>,
    pub source: Cow<'static, str>,
}

impl ShaderInclude {
    pub const IDENTIFIER: &'static str = "#include";

    pub fn parse(mut source: &str, path: Option<&Path>) -> Result<Self, ShaderProcessorError> {
        let mut includes = HashSet::new();

        let mut cleaned_source = String::new();
        while let Some(index) = source.find(Self::IDENTIFIER) {
            if let Some(comment) = source.find("/*") {
                if comment < index {
                    let comment_end = source.find("*/").unwrap_or(source.len());
                    cleaned_source += &source[..comment_end];
                    source = &source[comment_end..];

                    continue;
                }
            }

            if let Some(comment) = source.find("//") {
                if comment < index {
                    let comment_end = source.find("\n").unwrap_or(source.len());
                    cleaned_source += &source[..comment_end];
                    source = &source[comment_end..];
                    continue;
                }
            }

            cleaned_source += &source[..index];
            source = &source[index + Self::IDENTIFIER.len()..];

            let end = source.find('\n').unwrap_or(source.len());

            let path_source = source[..end].trim();
            includes.insert(ShaderIncludePath::parse(path_source, path)?);

            source = &source[end..];
        }

        cleaned_source += source;

        Ok(Self {
            includes,
            source: cleaned_source.into(),
        })
    }
}

impl From<&'static str> for ShaderInclude {
    fn from(source: &'static str) -> Self {
        Self {
            includes: HashSet::new(),
            source: source.into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ShaderProcessor {
    includes: HashMap<ShaderIncludePath<'static>, ShaderInclude>,
    modified: HashMap<ShaderIncludePath<'static>, SystemTime>,
}

impl ShaderProcessor {
    fn verify_local(&mut self, path: &Path) -> Result<(), Error> {
        let include_path = ShaderIncludePath::Local(Cow::Borrowed(path));

        if !self.includes.contains_key(&include_path) {
            if !path.exists() {
                return Err(ShaderProcessorError::InvalidLocal(path.to_path_buf()).into());
            }

            let source = fs::read_to_string(path)?;
            let shader_include = ShaderInclude::parse(&source, Some(path))?;

            let include_path = ShaderIncludePath::Local(path.to_path_buf().into());
            self.includes
                .insert(include_path.clone(), shader_include.clone());

            for _include_path in shader_include.includes.iter() {
                match self.verify_include_path(_include_path) {
                    Err(err) => {
                        self.includes.remove(&include_path);
                        return Err(err);
                    }
                    _ => {}
                }
            }

            let modified = path.metadata()?.modified()?;
            self.modified.insert(include_path, modified);
        }

        Ok(())
    }

    fn verify_include_path(&mut self, include_path: &ShaderIncludePath<'_>) -> Result<(), Error> {
        match include_path {
            ShaderIncludePath::Global(global_path) if !self.includes.contains_key(include_path) => {
                return Err(ShaderProcessorError::InvalidGlobal(global_path.clone()).into());
            }
            ShaderIncludePath::Local(local_path) => {
                self.verify_local(local_path)?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn empty() -> Self {
        Self {
            includes: HashMap::new(),
            modified: HashMap::new(),
        }
    }

    pub fn new() -> Self {
        let mut this = Self::empty();
        this.insert_global("kiln/uniforms", include_str!("include/uniforms.wgsl"), None)
            .unwrap();
        this.insert_global("kiln/ray", include_str!("include/ray.wgsl"), None)
            .unwrap();
        this.insert_global("kiln/input", include_str!("include/input.wgsl"), None)
            .unwrap();
        this.insert_global("kiln/camera", include_str!("include/camera.wgsl"), None)
            .unwrap();
        this.insert_global("kiln/post", include_str!("include/post.wgsl"), None)
            .unwrap();

        this
    }

    pub fn invalidate_locals(&mut self) {
        self.includes.retain(|k, _| !k.is_local());
    }

    pub fn insert_global(
        &mut self,
        include_path: &str,
        source: &str,
        path: Option<&Path>,
    ) -> Result<(), Error> {
        ShaderIncludePath::verify_path(&include_path)?;
        let include_path = ShaderIncludePath::Global(String::from(include_path));
        let shader_include = ShaderInclude::parse(source, path)?;

        for include_path in shader_include.includes.iter() {
            self.verify_include_path(&include_path)?;
        }

        self.includes.insert(include_path, shader_include);

        Ok(())
    }

    pub fn process(&mut self, path: &Path) -> Result<String, Error> {
        let path = fs::canonicalize(path)?;
        self.verify_local(&path)?;
        let include_path = ShaderIncludePath::Local(Cow::Borrowed(&path));

        let mut queue = VecDeque::new();
        let mut included = HashSet::new();
        let mut visited = HashSet::new();
        queue.push_back(include_path);

        let mut processed_shader = String::new();

        while let Some(include_path) = queue.pop_front() {
            if !visited.insert(include_path.clone()) {
                return Err(ShaderProcessorError::CyclicInclude.into());
            }

            let include = &self.includes[&include_path];

            let mut dependencies_included = true;

            for _include_path in include.includes.iter() {
                if !included.contains(_include_path) {
                    if !queue.contains(_include_path) {
                        queue.push_back(_include_path.clone());
                    }

                    dependencies_included = false;
                }
            }

            if !dependencies_included {
                queue.push_back(include_path);
                continue;
            }

            visited.clear();

            processed_shader += &include.source;
            included.insert(include_path);
        }

        Ok(processed_shader)
    }
}

#[derive(Debug, Error)]
pub enum ShaderProcessorError {
    #[error("unexpected opening delimiter in path '{0}', expected '<' or '\"'")]
    BadStart(char),
    #[error("unexpected closing delimiter in path, expected '{0}''")]
    BadEnd(char),
    #[error("bad character '{0}'")]
    BadChar(char),
    #[error("invalid global '{0}'")]
    InvalidGlobal(String),
    #[error("invalid local '{0}'")]
    InvalidLocal(PathBuf),
    #[error("cyclic include")]
    CyclicInclude,
    #[error("expected include path, eg. '#include <my_global>' or '#include \"my_local\"'")]
    ExpectedPath,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shader_include() {
        let source = r#"#include <input>
#include "help"

struct Foo {
    a: i32,
    b: f32,
}"#;

        let include = ShaderInclude::parse(source, Some(Path::new(""))).unwrap();

        assert_eq!(
            include.source,
            "\n\n\nstruct Foo {\n    a: i32,\n    b: f32,\n}"
        );
    }
}
