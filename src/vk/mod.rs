#![allow(non_snake_case, non_camel_case_types)]

#[macro_use] mod helpers;
mod consts;
mod enums;
mod flags;
mod types;
mod structs;
mod unions;
mod fns;
mod pfns;
pub mod ext;
pub mod khr;

pub use self::consts::*;
pub use self::enums::*;
pub use self::flags::*;
pub use self::types::*;
pub use self::structs::*;
pub use self::unions::*;
pub use self::fns::*;
pub use self::pfns::*;

pub fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}
