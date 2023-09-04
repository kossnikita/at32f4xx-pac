#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    pub ctrl1: CTRL1,
    #[doc = "0x04 - Control register 2"]
    pub ctrl2: CTRL2,
    #[doc = "0x08 - Subordinate TMR control register"]
    pub stctrl: STCTRL,
    #[doc = "0x0c - Interrupt/DMA enable register"]
    pub iden: IDEN,
    #[doc = "0x10 - Interrupt status register"]
    pub ists: ISTS,
    #[doc = "0x14 - Software event register"]
    pub swevt: SWEVT,
    _reserved_6_cm1: [u8; 0x04],
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Channel control register"]
    pub cctrl: CCTRL,
    #[doc = "0x24 - Counter value"]
    pub cval: CVAL,
    #[doc = "0x28 - Divider value"]
    pub div: DIV,
    #[doc = "0x2c - Period value"]
    pub pr: PR,
    #[doc = "0x30 - Repetition of period value"]
    pub rpr: RPR,
    #[doc = "0x34..0x3c - Channel data register"]
    pub cdt: [CDT; 2],
    _reserved13: [u8; 0x08],
    #[doc = "0x44 - Brake register"]
    pub brk: BRK,
    #[doc = "0x48 - DMA control register"]
    pub dmactrl: DMACTRL,
    #[doc = "0x4c - DMA data register"]
    pub dmadt: DMADT,
}
impl RegisterBlock {
    #[doc = "0x18 - Channel input mode register 1"]
    #[inline(always)]
    pub const fn cm1_input(&self) -> &CM1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - Channel output mode register"]
    #[inline(always)]
    pub const fn cm1_output(&self) -> &CM1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x34 - Channel data register"]
    #[inline(always)]
    pub fn c1dt(&self) -> &CDT {
        &self.cdt[0]
    }
    #[doc = "0x38 - Channel data register"]
    #[inline(always)]
    pub fn c2dt(&self) -> &CDT {
        &self.cdt[1]
    }
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
#[doc = "STCTRL (rw) register accessor: Subordinate TMR control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stctrl`]
module"]
pub type STCTRL = crate::Reg<stctrl::STCTRL_SPEC>;
#[doc = "Subordinate TMR control register"]
pub mod stctrl;
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
#[doc = "CM1_OUTPUT (rw) register accessor: Channel output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm1_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm1_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cm1_output`]
module"]
pub type CM1_OUTPUT = crate::Reg<cm1_output::CM1_OUTPUT_SPEC>;
#[doc = "Channel output mode register"]
pub mod cm1_output;
#[doc = "CM1_INPUT (rw) register accessor: Channel input mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm1_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm1_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cm1_input`]
module"]
pub type CM1_INPUT = crate::Reg<cm1_input::CM1_INPUT_SPEC>;
#[doc = "Channel input mode register 1"]
pub mod cm1_input;
#[doc = "CCTRL (rw) register accessor: Channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cctrl`]
module"]
pub type CCTRL = crate::Reg<cctrl::CCTRL_SPEC>;
#[doc = "Channel control register"]
pub mod cctrl;
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
#[doc = "RPR (rw) register accessor: Repetition of period value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rpr`]
module"]
pub type RPR = crate::Reg<rpr::RPR_SPEC>;
#[doc = "Repetition of period value"]
pub mod rpr;
#[doc = "CDT (rw) register accessor: Channel data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cdt`]
module"]
pub type CDT = crate::Reg<cdt::CDT_SPEC>;
#[doc = "Channel data register"]
pub mod cdt;
#[doc = "BRK (rw) register accessor: Brake register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`brk`]
module"]
pub type BRK = crate::Reg<brk::BRK_SPEC>;
#[doc = "Brake register"]
pub mod brk;
#[doc = "DMACTRL (rw) register accessor: DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmactrl`]
module"]
pub type DMACTRL = crate::Reg<dmactrl::DMACTRL_SPEC>;
#[doc = "DMA control register"]
pub mod dmactrl;
#[doc = "DMADT (rw) register accessor: DMA data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmadt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmadt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmadt`]
module"]
pub type DMADT = crate::Reg<dmadt::DMADT_SPEC>;
#[doc = "DMA data register"]
pub mod dmadt;
