#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    sts: STS,
    ests: ESTS,
    iena: IENA,
    ists: ISTS,
    iclr: ICLR,
    scr: SCR,
    sur: SUR,
    cwst: CWST,
    cwsz: CWSZ,
    dt: DT,
    _reserved11: [u8; 0x14],
    actrl: ACTRL,
    _reserved12: [u8; 0x04],
    hscf: HSCF,
    vscf: VSCF,
    frf: FRF,
    bth: BTH,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &STS {
        &self.sts
    }
    #[doc = "0x08 - Event status register"]
    #[inline(always)]
    pub const fn ests(&self) -> &ESTS {
        &self.ests
    }
    #[doc = "0x0c - interrupt enable register"]
    #[inline(always)]
    pub const fn iena(&self) -> &IENA {
        &self.iena
    }
    #[doc = "0x10 - Interrupt status register"]
    #[inline(always)]
    pub const fn ists(&self) -> &ISTS {
        &self.ists
    }
    #[doc = "0x14 - Interrupt clear register"]
    #[inline(always)]
    pub const fn iclr(&self) -> &ICLR {
        &self.iclr
    }
    #[doc = "0x18 - Synchronization code register"]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    #[doc = "0x1c - Synchronization unmask register"]
    #[inline(always)]
    pub const fn sur(&self) -> &SUR {
        &self.sur
    }
    #[doc = "0x20 - Crop window start"]
    #[inline(always)]
    pub const fn cwst(&self) -> &CWST {
        &self.cwst
    }
    #[doc = "0x24 - Crop window size"]
    #[inline(always)]
    pub const fn cwsz(&self) -> &CWSZ {
        &self.cwsz
    }
    #[doc = "0x28 - Data register"]
    #[inline(always)]
    pub const fn dt(&self) -> &DT {
        &self.dt
    }
    #[doc = "0x40 - Advanced Control register"]
    #[inline(always)]
    pub const fn actrl(&self) -> &ACTRL {
        &self.actrl
    }
    #[doc = "0x48 - Horizontal scaling control flow"]
    #[inline(always)]
    pub const fn hscf(&self) -> &HSCF {
        &self.hscf
    }
    #[doc = "0x4c - Vertical scaling control flow"]
    #[inline(always)]
    pub const fn vscf(&self) -> &VSCF {
        &self.vscf
    }
    #[doc = "0x50 - Frame rate flow"]
    #[inline(always)]
    pub const fn frf(&self) -> &FRF {
        &self.frf
    }
    #[doc = "0x54 - Binarization threshold"]
    #[inline(always)]
    pub const fn bth(&self) -> &BTH {
        &self.bth
    }
}
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "STS (r) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`] module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "status register"]
pub mod sts;
#[doc = "ESTS (r) register accessor: Event status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ests::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ests`] module"]
pub type ESTS = crate::Reg<ests::ESTS_SPEC>;
#[doc = "Event status register"]
pub mod ests;
#[doc = "IENA (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`iena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iena`] module"]
pub type IENA = crate::Reg<iena::IENA_SPEC>;
#[doc = "interrupt enable register"]
pub mod iena;
#[doc = "ISTS (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ists::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ists`] module"]
pub type ISTS = crate::Reg<ists::ISTS_SPEC>;
#[doc = "Interrupt status register"]
pub mod ists;
#[doc = "ICLR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`] module"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod iclr;
#[doc = "SCR (rw) register accessor: Synchronization code register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Synchronization code register"]
pub mod scr;
#[doc = "SUR (rw) register accessor: Synchronization unmask register\n\nYou can [`read`](crate::Reg::read) this register and get [`sur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sur`] module"]
pub type SUR = crate::Reg<sur::SUR_SPEC>;
#[doc = "Synchronization unmask register"]
pub mod sur;
#[doc = "CWST (rw) register accessor: Crop window start\n\nYou can [`read`](crate::Reg::read) this register and get [`cwst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwst`] module"]
pub type CWST = crate::Reg<cwst::CWST_SPEC>;
#[doc = "Crop window start"]
pub mod cwst;
#[doc = "CWSZ (rw) register accessor: Crop window size\n\nYou can [`read`](crate::Reg::read) this register and get [`cwsz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwsz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwsz`] module"]
pub type CWSZ = crate::Reg<cwsz::CWSZ_SPEC>;
#[doc = "Crop window size"]
pub mod cwsz;
#[doc = "DT (r) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`] module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "Data register"]
pub mod dt;
#[doc = "ACTRL (rw) register accessor: Advanced Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`actrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actrl`] module"]
pub type ACTRL = crate::Reg<actrl::ACTRL_SPEC>;
#[doc = "Advanced Control register"]
pub mod actrl;
#[doc = "HSCF (rw) register accessor: Horizontal scaling control flow\n\nYou can [`read`](crate::Reg::read) this register and get [`hscf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hscf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hscf`] module"]
pub type HSCF = crate::Reg<hscf::HSCF_SPEC>;
#[doc = "Horizontal scaling control flow"]
pub mod hscf;
#[doc = "VSCF (rw) register accessor: Vertical scaling control flow\n\nYou can [`read`](crate::Reg::read) this register and get [`vscf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vscf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vscf`] module"]
pub type VSCF = crate::Reg<vscf::VSCF_SPEC>;
#[doc = "Vertical scaling control flow"]
pub mod vscf;
#[doc = "FRF (rw) register accessor: Frame rate flow\n\nYou can [`read`](crate::Reg::read) this register and get [`frf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frf`] module"]
pub type FRF = crate::Reg<frf::FRF_SPEC>;
#[doc = "Frame rate flow"]
pub mod frf;
#[doc = "BTH (rw) register accessor: Binarization threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`bth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bth`] module"]
pub type BTH = crate::Reg<bth::BTH_SPEC>;
#[doc = "Binarization threshold"]
pub mod bth;
