#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Command register"]
    pub cmd: CMD,
    #[doc = "0x04 - Division register"]
    pub div: DIV,
    #[doc = "0x08 - Reload register"]
    pub rld: RLD,
    #[doc = "0x0c - Status register"]
    pub sts: STS,
    #[doc = "0x10 - Window register"]
    pub win: WIN,
}
#[doc = "CMD (w) register accessor: Command register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command register"]
pub mod cmd;
#[doc = "DIV (rw) register accessor: Division register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`div`]
module"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Division register"]
pub mod div;
#[doc = "RLD (rw) register accessor: Reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rld`]
module"]
pub type RLD = crate::Reg<rld::RLD_SPEC>;
#[doc = "Reload register"]
pub mod rld;
#[doc = "STS (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "Status register"]
pub mod sts;
#[doc = "WIN (r) register accessor: Window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`win`]
module"]
pub type WIN = crate::Reg<win::WIN_SPEC>;
#[doc = "Window register"]
pub mod win;
