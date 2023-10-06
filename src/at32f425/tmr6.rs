#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    pub ctrl1: CTRL1,
    #[doc = "0x04 - Control register 2"]
    pub ctrl2: CTRL2,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Interrupt/DMA enable register"]
    pub iden: IDEN,
    #[doc = "0x10 - Interrupt status register"]
    pub ists: ISTS,
    #[doc = "0x14 - Software event register"]
    pub swevt: SWEVT,
    _reserved5: [u8; 0x0c],
    #[doc = "0x24 - Counter value"]
    pub cval: CVAL,
    _reserved6: [u8; 0x02],
    #[doc = "0x28 - Divider value"]
    pub div: DIV,
    _reserved7: [u8; 0x02],
    #[doc = "0x2c - Period value"]
    pub pr: PR,
}
#[doc = "CTRL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control register 2"]
pub mod ctrl2;
#[doc = "IDEN (rw) register accessor: Interrupt/DMA enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iden::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iden::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iden`]
module"]
pub type IDEN = crate::Reg<iden::IDEN_SPEC>;
#[doc = "Interrupt/DMA enable register"]
pub mod iden;
#[doc = "ISTS (rw) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ists::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ists::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ists`]
module"]
pub type ISTS = crate::Reg<ists::ISTS_SPEC>;
#[doc = "Interrupt status register"]
pub mod ists;
#[doc = "SWEVT (rw) register accessor: Software event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swevt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swevt`]
module"]
pub type SWEVT = crate::Reg<swevt::SWEVT_SPEC>;
#[doc = "Software event register"]
pub mod swevt;
#[doc = "CVAL (rw) register accessor: Counter value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cval`]
module"]
pub type CVAL = crate::Reg<cval::CVAL_SPEC>;
#[doc = "Counter value"]
pub mod cval;
#[doc = "DIV (rw) register accessor: Divider value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`div`]
module"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Divider value"]
pub mod div;
#[doc = "PR (rw) register accessor: Period value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pr`]
module"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Period value"]
pub mod pr;
