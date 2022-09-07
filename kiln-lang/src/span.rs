use std::ops::{Deref, DerefMut};

use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SourceId {
    uuid: Uuid,
}

impl SourceId {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    index: usize,
    length: usize,
    source: SourceId,
}

impl Span {
    pub const fn new(index: usize, length: usize, source: SourceId) -> Self {
        Self {
            index,
            length,
            source,
        }
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub const fn length(&self) -> usize {
        self.length
    }

    pub const fn source(&self) -> SourceId {
        self.source
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Spanned<T> {
    value: T,
    span: Span,
}

impl<T> Spanned<T> {
    pub const fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }

    pub const fn span(&self) -> Span {
        self.span
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub trait IntoSpanned: Sized {
    fn spanned(self, span: Span) -> Spanned<Self> {
        Spanned::new(self, span)
    }
}

impl<T> IntoSpanned for T {}
