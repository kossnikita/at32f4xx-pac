#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfgr: CFGR,
    omode: OMODE,
    odrvr: ODRVR,
    pull: PULL,
    idt: IDT,
    odt: ODT,
    scr: SCR,
    wpr: WPR,
    muxl: MUXL,
    muxh: MUXH,
    clr: CLR,
    togr: TOGR,
    _reserved12: [u8; 0x0c],
    hdrv: HDRV,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x04 - GPIO output mode register"]
    #[inline(always)]
    pub const fn omode(&self) -> &OMODE {
        &self.omode
    }
    #[doc = "0x08 - GPIO drive capability register"]
    #[inline(always)]
    pub const fn odrvr(&self) -> &ODRVR {
        &self.odrvr
    }
    #[doc = "0x0c - GPIO pull-up/pull-down register"]
    #[inline(always)]
    pub const fn pull(&self) -> &PULL {
        &self.pull
    }
    #[doc = "0x10 - GPIO input data register"]
    #[inline(always)]
    pub const fn idt(&self) -> &IDT {
        &self.idt
    }
    #[doc = "0x14 - GPIO output data register"]
    #[inline(always)]
    pub const fn odt(&self) -> &ODT {
        &self.odt
    }
    #[doc = "0x18 - Port bit set/clear register"]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    #[doc = "0x1c - Port write protect register"]
    #[inline(always)]
    pub const fn wpr(&self) -> &WPR {
        &self.wpr
    }
    #[doc = "0x20 - GPIO muxing function low register"]
    #[inline(always)]
    pub const fn muxl(&self) -> &MUXL {
        &self.muxl
    }
    #[doc = "0x24 - GPIO muxing function high register"]
    #[inline(always)]
    pub const fn muxh(&self) -> &MUXH {
        &self.muxh
    }
    #[doc = "0x28 - GPIO bit reset register"]
    #[inline(always)]
    pub const fn clr(&self) -> &CLR {
        &self.clr
    }
    #[doc = "0x2c - GPIO bit toggle register"]
    #[inline(always)]
    pub const fn togr(&self) -> &TOGR {
        &self.togr
    }
    #[doc = "0x3c - Huge current driver"]
    #[inline(always)]
    pub const fn hdrv(&self) -> &HDRV {
        &self.hdrv
    }
}
#[doc = "CFGR (rw) register accessor: GPIO configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "GPIO configuration register"]
pub mod cfgr;
#[doc = "OMODE (rw) register accessor: GPIO output mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`omode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`omode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@omode`] module"]
pub type OMODE = crate::Reg<omode::OMODE_SPEC>;
#[doc = "GPIO output mode register"]
pub mod omode;
#[doc = "ODRVR (rw) register accessor: GPIO drive capability register\n\nYou can [`read`](crate::Reg::read) this register and get [`odrvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odrvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odrvr`] module"]
pub type ODRVR = crate::Reg<odrvr::ODRVR_SPEC>;
#[doc = "GPIO drive capability register"]
pub mod odrvr;
#[doc = "PULL (rw) register accessor: GPIO pull-up/pull-down register\n\nYou can [`read`](crate::Reg::read) this register and get [`pull::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pull::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pull`] module"]
pub type PULL = crate::Reg<pull::PULL_SPEC>;
#[doc = "GPIO pull-up/pull-down register"]
pub mod pull;
#[doc = "IDT (r) register accessor: GPIO input data register\n\nYou can [`read`](crate::Reg::read) this register and get [`idt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idt`] module"]
pub type IDT = crate::Reg<idt::IDT_SPEC>;
#[doc = "GPIO input data register"]
pub mod idt;
#[doc = "ODT (rw) register accessor: GPIO output data register\n\nYou can [`read`](crate::Reg::read) this register and get [`odt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odt`] module"]
pub type ODT = crate::Reg<odt::ODT_SPEC>;
#[doc = "GPIO output data register"]
pub mod odt;
#[doc = "SCR (w) register accessor: Port bit set/clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Port bit set/clear register"]
pub mod scr;
#[doc = "WPR (rw) register accessor: Port write protect register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpr`] module"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "Port write protect register"]
pub mod wpr;
#[doc = "MUXL (rw) register accessor: GPIO muxing function low register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxl`] module"]
pub type MUXL = crate::Reg<muxl::MUXL_SPEC>;
#[doc = "GPIO muxing function low register"]
pub mod muxl;
#[doc = "MUXH (rw) register accessor: GPIO muxing function high register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxh`] module"]
pub type MUXH = crate::Reg<muxh::MUXH_SPEC>;
#[doc = "GPIO muxing function high register"]
pub mod muxh;
#[doc = "CLR (w) register accessor: GPIO bit reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`] module"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "GPIO bit reset register"]
pub mod clr;
#[doc = "TOGR (w) register accessor: GPIO bit toggle register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`togr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@togr`] module"]
pub type TOGR = crate::Reg<togr::TOGR_SPEC>;
#[doc = "GPIO bit toggle register"]
pub mod togr;
#[doc = "HDRV (rw) register accessor: Huge current driver\n\nYou can [`read`](crate::Reg::read) this register and get [`hdrv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdrv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdrv`] module"]
pub type HDRV = crate::Reg<hdrv::HDRV_SPEC>;
#[doc = "Huge current driver"]
pub mod hdrv;
