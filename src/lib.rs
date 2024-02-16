// [[file:../grep-reader.note::90c4eed8][90c4eed8]]
#![deny(warnings)]
#![deny(clippy::all)]
// 90c4eed8 ends here

// [[file:../grep-reader.note::78a25f58][78a25f58]]
mod grep;
// 78a25f58 ends here

// [[file:../grep-reader.note::6579be04][6579be04]]
use std::path::Path;

use anyhow::bail;
use anyhow::ensure;
use anyhow::Result;
// 6579be04 ends here

// [[file:../grep-reader.note::cea1a02d][cea1a02d]]
pub use self::grep::*;
// cea1a02d ends here
