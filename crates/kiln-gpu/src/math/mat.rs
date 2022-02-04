use crate::{Vec2, Vec3, Vec4};

#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mat2x2<T> {
    pub x: Vec2<T>,
    pub y: Vec2<T>,
}

impl<T> Mat2x2<T> {
    pub const fn new(x: Vec2<T>, y: Vec2<T>) -> Self {
        Self { x, y }
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mat3x3<T> {
    pub x: Vec3<T>,
    pub y: Vec3<T>,
    pub z: Vec3<T>,
}

impl<T> Mat3x3<T> {
    pub const fn new(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Self {
        Self { x, y, z }
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mat4x4<T> {
    pub x: Vec4<T>,
    pub y: Vec4<T>,
    pub z: Vec4<T>,
    pub w: Vec4<T>,
}

impl<T> Mat4x4<T> {
    pub const fn new(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T>) -> Self {
        Self { x, y, z, w }
    }
}
