// SPDX-FileCopyrightText: © The `quake3` Rust crate authors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Info strings/maps.
//!
//! Quake3 has so called "info strings" which are a C string consisting
//! of key/value pairs separated by `\` backslashes;
//! see [`code/qcommon/q_shared.c`](https://github.com/ioquake/ioq3/blob/main/code/qcommon/q_shared.c).

/// Key string for [`Info`].
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Key(Vec<u8>);

impl Key {
    /// Creates a new key string from a container of bytes.
    pub fn new<T: Into<Vec<u8>>>(t: T) -> Result<Self, ()> {
        let bytes = t.into();

        Ok(Self(bytes))
    }
}

impl<I: std::slice::SliceIndex<[u8]>> std::ops::Index<I> for Key {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        std::ops::Index::index(&self.0, index)
    }
}

/// Value string for [`Info`].
#[derive(Debug, PartialEq, Eq)]
pub struct Value(Vec<u8>);

impl Value {
    /// Creates a new value string from a container of bytes.
    pub fn new<T: Into<Vec<u8>>>(t: T) -> Result<Self, ()> {
        let bytes = t.into();

        Ok(Self(bytes))
    }
}

impl<I: std::slice::SliceIndex<[u8]>> std::ops::Index<I> for Value {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        std::ops::Index::index(&self.0, index)
    }
}

/// Key/value map with string format.
#[derive(Debug, PartialEq, Eq)]
pub struct Info(indexmap::IndexMap<Key, Value>);

impl Info {
    /// Creates an empty info map.
    pub fn new() -> Self {
        Self(indexmap::IndexMap::new())
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, key: Key, value: Value) {
        self.0.insert(key, value);
    }

    /// An iterator visiting all key-value pairs in their order.
    pub fn iter(&self) -> impl core::iter::Iterator<Item = (&Key, &Value)> {
        self.0.iter()
    }
}

impl Default for Info {
    /// Returns an empty info string/map.
    fn default() -> Self {
        Self::new()
    }
}
