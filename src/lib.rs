#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "at32a403a")]
pub mod at32a403a;
#[cfg(feature = "at32f402")]
pub mod at32f402;
#[cfg(feature = "at32f403")]
pub mod at32f403;
#[cfg(feature = "at32f403a")]
pub mod at32f403a;
#[cfg(feature = "at32f405")]
pub mod at32f405;
#[cfg(feature = "at32f407")]
pub mod at32f407;
#[cfg(feature = "at32f413")]
pub mod at32f413;
#[cfg(feature = "at32f415")]
pub mod at32f415;
#[cfg(feature = "at32f421")]
pub mod at32f421;
#[cfg(feature = "at32f423")]
pub mod at32f423;
#[cfg(feature = "at32f425")]
pub mod at32f425;
#[cfg(feature = "at32f435")]
pub mod at32f435;
#[cfg(feature = "at32f437")]
pub mod at32f437;
#[cfg(feature = "at32wb415")]
pub mod at32wb415;