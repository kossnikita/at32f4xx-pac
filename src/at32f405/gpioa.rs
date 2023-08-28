#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x04 - GPIO output mode register"]
    pub omode: OMODE,
    #[doc = "0x08 - GPIO drive capability register"]
    pub odrvr: ODRVR,
    #[doc = "0x0c - GPIO pull-up/pull-down register"]
    pub pull: PULL,
    #[doc = "0x10 - GPIO input data register"]
    pub idt: IDT,
    #[doc = "0x14 - GPIO output data register"]
    pub odt: ODT,
    #[doc = "0x18 - Port bit set/clear register"]
    pub scr: SCR,
    #[doc = "0x1c - Port write protect register"]
    pub wpr: WPR,
    #[doc = "0x20 - GPIO muxing function low register"]
    pub muxl: MUXL,
    #[doc = "0x24 - GPIO muxing function high register"]
    pub muxh: MUXH,
    #[doc = "0x28 - GPIO bit reset register"]
    pub clr: CLR,
    #[doc = "0x2c - GPIO bit toggle register"]
    pub togr: TOGR,
    _reserved12: [u8; 0x0c],
    #[doc = "0x3c - Huge current driver"]
    pub hdrv: HDRV,
}
#[doc = "CFGR (rw) register accessor: GPIO configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "GPIO configuration register"]
pub mod cfgr;
#[doc = "OMODE (rw) register accessor: GPIO output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`omode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`omode`]
module"]
pub type OMODE = crate::Reg<omode::OMODE_SPEC>;
#[doc = "GPIO output mode register"]
pub mod omode;
#[doc = "ODRVR (rw) register accessor: GPIO drive capability register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odrvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odrvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`odrvr`]
module"]
pub type ODRVR = crate::Reg<odrvr::ODRVR_SPEC>;
#[doc = "GPIO drive capability register"]
pub mod odrvr;
#[doc = "PULL (rw) register accessor: GPIO pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pull::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pull::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pull`]
module"]
pub type PULL = crate::Reg<pull::PULL_SPEC>;
#[doc = "GPIO pull-up/pull-down register"]
pub mod pull;
#[doc = "IDT (r) register accessor: GPIO input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idt`]
module"]
pub type IDT = crate::Reg<idt::IDT_SPEC>;
#[doc = "GPIO input data register"]
pub mod idt;
#[doc = "ODT (rw) register accessor: GPIO output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`odt`]
module"]
pub type ODT = crate::Reg<odt::ODT_SPEC>;
#[doc = "GPIO output data register"]
pub mod odt;
#[doc = "SCR (w) register accessor: Port bit set/clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scr`]
module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Port bit set/clear register"]
pub mod scr;
#[doc = "WPR (rw) register accessor: Port write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpr`]
module"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "Port write protect register"]
pub mod wpr;
#[doc = "MUXL (rw) register accessor: GPIO muxing function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxl`]
module"]
pub type MUXL = crate::Reg<muxl::MUXL_SPEC>;
#[doc = "GPIO muxing function low register"]
pub mod muxl;
#[doc = "MUXH (rw) register accessor: GPIO muxing function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxh`]
module"]
pub type MUXH = crate::Reg<muxh::MUXH_SPEC>;
#[doc = "GPIO muxing function high register"]
pub mod muxh;
#[doc = "CLR (w) register accessor: GPIO bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr`]
module"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "GPIO bit reset register"]
pub mod clr;
#[doc = "TOGR (w) register accessor: GPIO bit toggle register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`togr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`togr`]
module"]
pub type TOGR = crate::Reg<togr::TOGR_SPEC>;
#[doc = "GPIO bit toggle register"]
pub mod togr;
#[doc = "HDRV (rw) register accessor: Huge current driver\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdrv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdrv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hdrv`]
module"]
pub type HDRV = crate::Reg<hdrv::HDRV_SPEC>;
#[doc = "Huge current driver"]
pub mod hdrv;
