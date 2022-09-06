use std::{
    fs,
    path::{Path, PathBuf},
};

use glam::{Mat4, Vec3};
use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "kebab-case")]
pub struct Manifest {
    pub project: ManifestProject,
    #[serde(default)]
    pub camera: ManifestCamera,
    #[serde(default)]
    pub shaders: LinkedHashMap<String, ManifestShader>,
}

impl Manifest {
    pub const DEFAULT_PATH: &'static str = "Kiln.toml";

    pub fn load(path: &Path) -> Result<Self> {
        let source = fs::read_to_string(path)?;
        Ok(toml::from_str(&source)?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "kebab-case")]
pub struct ManifestProject {
    pub name: String,
    pub author: Option<String>,
}

const fn default_direction() -> Vec3 {
    Vec3::Z
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "kebab-case")]
pub struct ManifestCamera {
    #[serde(default)]
    pub position: Vec3,
    #[serde(default = "default_direction")]
    pub direction: Vec3,
}

impl Default for ManifestCamera {
    fn default() -> Self {
        Self {
            position: Default::default(),
            direction: default_direction(),
        }
    }
}

impl ManifestCamera {
    pub fn view(&self) -> Mat4 {
        let translation = Mat4::from_translation(self.position);
        translation * Mat4::look_at_lh(Vec3::ZERO, self.direction.normalize_or_zero(), Vec3::Y)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "kebab-case")]
pub struct ManifestShader {
    pub fragment: Option<PathBuf>,
    pub vertex: Option<PathBuf>,
}
