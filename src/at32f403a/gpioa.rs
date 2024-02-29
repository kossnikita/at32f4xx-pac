#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfglr: CFGLR,
    cfghr: CFGHR,
    idt: IDT,
    odt: ODT,
    scr: SCR,
    clr: CLR,
    wpr: WPR,
    _reserved7: [u8; 0x20],
    hdrv: HDRV,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO function configurate low register"]
    #[inline(always)]
    pub const fn cfglr(&self) -> &CFGLR {
        &self.cfglr
    }
    #[doc = "0x04 - GPIO function configurate high register"]
    #[inline(always)]
    pub const fn cfghr(&self) -> &CFGHR {
        &self.cfghr
    }
    #[doc = "0x08 - Port input data register"]
    #[inline(always)]
    pub const fn idt(&self) -> &IDT {
        &self.idt
    }
    #[doc = "0x0c - Port output data register"]
    #[inline(always)]
    pub const fn odt(&self) -> &ODT {
        &self.odt
    }
    #[doc = "0x10 - Port bit set/clear register"]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    #[doc = "0x14 - Port bit reset register"]
    #[inline(always)]
    pub const fn clr(&self) -> &CLR {
        &self.clr
    }
    #[doc = "0x18 - Port write protect register"]
    #[inline(always)]
    pub const fn wpr(&self) -> &WPR {
        &self.wpr
    }
    #[doc = "0x3c - Port configuration driver register"]
    #[inline(always)]
    pub const fn hdrv(&self) -> &HDRV {
        &self.hdrv
    }
}
#[doc = "CFGLR (rw) register accessor: GPIO function configurate low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfglr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfglr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfglr`]
module"]
pub type CFGLR = crate::Reg<cfglr::CFGLR_SPEC>;
#[doc = "GPIO function configurate low register"]
pub mod cfglr;
#[doc = "CFGHR (rw) register accessor: GPIO function configurate high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfghr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfghr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfghr`]
module"]
pub type CFGHR = crate::Reg<cfghr::CFGHR_SPEC>;
#[doc = "GPIO function configurate high register"]
pub mod cfghr;
#[doc = "IDT (r) register accessor: Port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idt`]
module"]
pub type IDT = crate::Reg<idt::IDT_SPEC>;
#[doc = "Port input data register"]
pub mod idt;
#[doc = "ODT (rw) register accessor: Port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odt`]
module"]
pub type ODT = crate::Reg<odt::ODT_SPEC>;
#[doc = "Port output data register"]
pub mod odt;
#[doc = "SCR (w) register accessor: Port bit set/clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Port bit set/clear register"]
pub mod scr;
#[doc = "CLR (w) register accessor: Port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "Port bit reset register"]
pub mod clr;
#[doc = "WPR (rw) register accessor: Port write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpr`]
module"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "Port write protect register"]
pub mod wpr;
#[doc = "HDRV (rw) register accessor: Port configuration driver register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdrv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdrv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdrv`]
module"]
pub type HDRV = crate::Reg<hdrv::HDRV_SPEC>;
#[doc = "Port configuration driver register"]
pub mod hdrv;
