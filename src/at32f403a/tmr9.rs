#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl1: CTRL1,
    _reserved1: [u8; 0x04],
    stctrl: STCTRL,
    iden: IDEN,
    ists: ISTS,
    swevt: SWEVT,
    _reserved_5_cm1: [u8; 0x04],
    _reserved6: [u8; 0x04],
    cctrl: CCTRL,
    cval: CVAL,
    _reserved8: [u8; 0x02],
    div: DIV,
    _reserved9: [u8; 0x02],
    pr: PR,
    _reserved10: [u8; 0x06],
    cdt: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x08 - Subordinate TMR control register"]
    #[inline(always)]
    pub const fn stctrl(&self) -> &STCTRL {
        &self.stctrl
    }
    #[doc = "0x0c - Interrupt/DMA enable register"]
    #[inline(always)]
    pub const fn iden(&self) -> &IDEN {
        &self.iden
    }
    #[doc = "0x10 - Interrupt status register"]
    #[inline(always)]
    pub const fn ists(&self) -> &ISTS {
        &self.ists
    }
    #[doc = "0x14 - Software event register"]
    #[inline(always)]
    pub const fn swevt(&self) -> &SWEVT {
        &self.swevt
    }
    #[doc = "0x18 - Channel input mode register 1"]
    #[inline(always)]
    pub const fn cm1_input(&self) -> &CM1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - Channel output mode register"]
    #[inline(always)]
    pub const fn cm1_output(&self) -> &CM1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x20 - Channel control register"]
    #[inline(always)]
    pub const fn cctrl(&self) -> &CCTRL {
        &self.cctrl
    }
    #[doc = "0x24 - Counter value"]
    #[inline(always)]
    pub const fn cval(&self) -> &CVAL {
        &self.cval
    }
    #[doc = "0x28 - Divider value"]
    #[inline(always)]
    pub const fn div(&self) -> &DIV {
        &self.div
    }
    #[doc = "0x2c - Period value"]
    #[inline(always)]
    pub const fn pr(&self) -> &PR {
        &self.pr
    }
    #[doc = "0x34 - Channel data register"]
    #[inline(always)]
    pub const fn cdt(&self, n: usize) -> &CDT {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(52).add(4 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34 - Channel data register"]
    #[inline(always)]
    pub fn cdt_iter(&self) -> impl Iterator<Item = &CDT> {
        (0..2)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(52).add(4 * n).cast() })
    }
    #[doc = "0x34 - Channel data register"]
    #[inline(always)]
    pub const fn c1dt(&self) -> &CDT {
        self.cdt(0)
    }
    #[doc = "0x38 - Channel data register"]
    #[inline(always)]
    pub const fn c2dt(&self) -> &CDT {
        self.cdt(1)
    }
}
#[doc = "CTRL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctrl1;
#[doc = "STCTRL (rw) register accessor: Subordinate TMR control register\n\nYou can [`read`](crate::Reg::read) this register and get [`stctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stctrl`]
module"]
pub type STCTRL = crate::Reg<stctrl::STCTRL_SPEC>;
#[doc = "Subordinate TMR control register"]
pub mod stctrl;
#[doc = "IDEN (rw) register accessor: Interrupt/DMA enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`iden::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iden::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iden`]
module"]
pub type IDEN = crate::Reg<iden::IDEN_SPEC>;
#[doc = "Interrupt/DMA enable register"]
pub mod iden;
#[doc = "ISTS (rw) register accessor: Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ists::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ists::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ists`]
module"]
pub type ISTS = crate::Reg<ists::ISTS_SPEC>;
#[doc = "Interrupt status register"]
pub mod ists;
#[doc = "SWEVT (rw) register accessor: Software event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swevt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swevt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swevt`]
module"]
pub type SWEVT = crate::Reg<swevt::SWEVT_SPEC>;
#[doc = "Software event register"]
pub mod swevt;
#[doc = "CM1_OUTPUT (rw) register accessor: Channel output mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`cm1_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm1_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm1_output`]
module"]
pub type CM1_OUTPUT = crate::Reg<cm1_output::CM1_OUTPUT_SPEC>;
#[doc = "Channel output mode register"]
pub mod cm1_output;
#[doc = "CM1_INPUT (rw) register accessor: Channel input mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cm1_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm1_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm1_input`]
module"]
pub type CM1_INPUT = crate::Reg<cm1_input::CM1_INPUT_SPEC>;
#[doc = "Channel input mode register 1"]
pub mod cm1_input;
#[doc = "CCTRL (rw) register accessor: Channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cctrl`]
module"]
pub type CCTRL = crate::Reg<cctrl::CCTRL_SPEC>;
#[doc = "Channel control register"]
pub mod cctrl;
#[doc = "CVAL (rw) register accessor: Counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`cval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cval`]
module"]
pub type CVAL = crate::Reg<cval::CVAL_SPEC>;
#[doc = "Counter value"]
pub mod cval;
#[doc = "DIV (rw) register accessor: Divider value\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`]
module"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Divider value"]
pub mod div;
#[doc = "PR (rw) register accessor: Period value\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Period value"]
pub mod pr;
#[doc = "CDT (rw) register accessor: Channel data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdt`]
module"]
pub type CDT = crate::Reg<cdt::CDT_SPEC>;
#[doc = "Channel data register"]
pub mod cdt;
