#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO function configurate low register"]
    pub cfglr: CFGLR,
    #[doc = "0x04 - GPIO function configurate high register"]
    pub cfghr: CFGHR,
    #[doc = "0x08 - Port input data register"]
    pub idt: IDT,
    #[doc = "0x0c - Port output data register"]
    pub odt: ODT,
    #[doc = "0x10 - Port bit set/clear register"]
    pub scr: SCR,
    #[doc = "0x14 - Port bit reset register"]
    pub clr: CLR,
    #[doc = "0x18 - Port write protect register"]
    pub wpr: WPR,
    _reserved7: [u8; 0x20],
    #[doc = "0x3c - Port configuration driver register"]
    pub hdrv: HDRV,
}
#[doc = "CFGLR (rw) register accessor: GPIO function configurate low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfglr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfglr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfglr`]
module"]
pub type CFGLR = crate::Reg<cfglr::CFGLR_SPEC>;
#[doc = "GPIO function configurate low register"]
pub mod cfglr;
#[doc = "CFGHR (rw) register accessor: GPIO function configurate high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfghr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfghr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfghr`]
module"]
pub type CFGHR = crate::Reg<cfghr::CFGHR_SPEC>;
#[doc = "GPIO function configurate high register"]
pub mod cfghr;
#[doc = "IDT (r) register accessor: Port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idt`]
module"]
pub type IDT = crate::Reg<idt::IDT_SPEC>;
#[doc = "Port input data register"]
pub mod idt;
#[doc = "ODT (rw) register accessor: Port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`odt`]
module"]
pub type ODT = crate::Reg<odt::ODT_SPEC>;
#[doc = "Port output data register"]
pub mod odt;
#[doc = "SCR (w) register accessor: Port bit set/clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scr`]
module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Port bit set/clear register"]
pub mod scr;
#[doc = "CLR (w) register accessor: Port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr`]
module"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "Port bit reset register"]
pub mod clr;
#[doc = "WPR (rw) register accessor: Port write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wpr`]
module"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "Port write protect register"]
pub mod wpr;
#[doc = "HDRV (rw) register accessor: Port configuration driver register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdrv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdrv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hdrv`]
module"]
pub type HDRV = crate::Reg<hdrv::HDRV_SPEC>;
#[doc = "Port configuration driver register"]
pub mod hdrv;
