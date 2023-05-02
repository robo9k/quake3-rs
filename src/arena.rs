// SPDX-FileCopyrightText: © The `quake3` Rust crate authors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Arenas (levels / maps).

use indexmap::IndexSet;

use crate::info::Info;

/// Newtype for untyped arena [`Info`](Info).
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct ArenaInfo(Info);

impl ArenaInfo {
    pub fn new(info: Info) -> Self {
        Self(info)
    }

    // delegate all of Info API..
}

type MapName = ();
type ColorString = ();
type BotName = ();
type FragLimit = ();
type TimeLimit = ();
type GameType = ();
type SpecialArenaTag = ();

/// Typed [`ArenaInfo`](ArenaInfo) for a map/level.
#[derive(Debug, PartialEq, Eq /*, Clone*/)]
pub struct Arena {
    /// Typed value for `map` key.
    map: MapName,
    /// Typed value for `longname` key.
    name: ColorString,
    /// Typed value for `bots` key.
    bots: Vec<BotName>,
    /// Typed value for `fraglimit` key.
    fraglimit: FragLimit,
    /// Typed value for `timelimit` key.
    timelimit: TimeLimit,
    /// Typed value for `type` key.
    r#type: IndexSet<GameType>,
    /// Typed value for `special` key.
    special: SpecialArenaTag,

    /// Other untyped info key/value pairs.
    info: Info,
}

// TODO: builder
impl Arena {
    pub fn new(info: ArenaInfo) -> Result<Self, ()> {
        todo!();
    }

    // subset of Info API..

    pub fn map(&self) -> &MapName {
        &self.map
    }

    pub fn name(&self) -> &ColorString {
        &self.name
    }

    pub fn bots(&self) -> () {
        todo!();
    }

    pub fn fraglimit(&self) -> &FragLimit {
        &self.fraglimit
    }

    pub fn timelimit(&self) -> &TimeLimit {
        &self.timelimit
    }

    pub fn r#type(&self) -> () {
        todo!();
    }

    pub fn special(&self) -> &SpecialArenaTag {
        &self.special
    }
}
