// [[file:../grep-reader.note::*lib.rs][lib.rs:1]]
#![deny(warnings)]
#![deny(clippy::all)]
// lib.rs:1 ends here

// [[file:../grep-reader.note::*mods][mods:1]]

// mods:1 ends here

// [[file:../grep-reader.note::ae06ab68][ae06ab68]]
#[cfg(feature = "adhoc")]
/// Docs for local mods
pub mod docs {
    macro_rules! export_doc {
        ($l:ident) => {
            pub mod $l {
                pub use crate::$l::*;
            }
        };
    }

    // export_doc!(codec);
}
// ae06ab68 ends here
