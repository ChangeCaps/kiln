use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("path doesn't exist: {0}")]
    InvalidPath(PathBuf),
    #[error("error loading manifest: {0}")]
    Manifest(#[from] toml::de::Error),
    #[error("surface error: {0}")]
    Surface(#[from] wgpu::SurfaceError),
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("os error: {0}")]
    Os(#[from] winit::error::OsError),
}

pub type Result<T> = std::result::Result<T, Error>;
