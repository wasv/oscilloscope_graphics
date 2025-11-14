#![no_std]

mod prelude {
    #![allow(unused_imports)]
    pub extern crate alloc;

    pub use alloc::borrow::ToOwned;
    pub use alloc::boxed::Box;
    pub use alloc::collections::{BTreeMap, VecDeque};
    pub use alloc::format;
    pub use alloc::string::String;
    pub use alloc::string::ToString;
    pub use alloc::vec::Vec;
    pub use core::str::FromStr;
}

/// Abstract signals and output
pub mod signal;

/// Line-drawing signal
pub mod linedraw;

/// Vector graphics description language
pub mod vgdl;
