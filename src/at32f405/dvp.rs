#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - status register"]
    pub sts: STS,
    #[doc = "0x08 - Event status register"]
    pub ests: ESTS,
    #[doc = "0x0c - interrupt enable register"]
    pub iena: IENA,
    #[doc = "0x10 - Interrupt status register"]
    pub ists: ISTS,
    #[doc = "0x14 - Interrupt clear register"]
    pub iclr: ICLR,
    #[doc = "0x18 - Synchronization code register"]
    pub scr: SCR,
    #[doc = "0x1c - Synchronization unmask register"]
    pub sur: SUR,
    #[doc = "0x20 - Crop window start"]
    pub cwst: CWST,
    #[doc = "0x24 - Crop window size"]
    pub cwsz: CWSZ,
    #[doc = "0x28 - Data register"]
    pub dt: DT,
    _reserved11: [u8; 0x14],
    #[doc = "0x40 - Advanced Control register"]
    pub actrl: ACTRL,
    _reserved12: [u8; 0x04],
    #[doc = "0x48 - Horizontal scaling control flow"]
    pub hscf: HSCF,
    #[doc = "0x4c - Vertical scaling control flow"]
    pub vscf: VSCF,
    #[doc = "0x50 - Frame rate flow"]
    pub frf: FRF,
    #[doc = "0x54 - Binarization threshold"]
    pub bth: BTH,
}
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "STS (r) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "status register"]
pub mod sts;
#[doc = "ESTS (r) register accessor: Event status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ests::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ests`]
module"]
pub type ESTS = crate::Reg<ests::ESTS_SPEC>;
#[doc = "Event status register"]
pub mod ests;
#[doc = "IENA (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iena`]
module"]
pub type IENA = crate::Reg<iena::IENA_SPEC>;
#[doc = "interrupt enable register"]
pub mod iena;
#[doc = "ISTS (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ists::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ists`]
module"]
pub type ISTS = crate::Reg<ists::ISTS_SPEC>;
#[doc = "Interrupt status register"]
pub mod ists;
#[doc = "ICLR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iclr`]
module"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod iclr;
#[doc = "SCR (rw) register accessor: Synchronization code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scr`]
module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Synchronization code register"]
pub mod scr;
#[doc = "SUR (rw) register accessor: Synchronization unmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sur`]
module"]
pub type SUR = crate::Reg<sur::SUR_SPEC>;
#[doc = "Synchronization unmask register"]
pub mod sur;
#[doc = "CWST (rw) register accessor: Crop window start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cwst`]
module"]
pub type CWST = crate::Reg<cwst::CWST_SPEC>;
#[doc = "Crop window start"]
pub mod cwst;
#[doc = "CWSZ (rw) register accessor: Crop window size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwsz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwsz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cwsz`]
module"]
pub type CWSZ = crate::Reg<cwsz::CWSZ_SPEC>;
#[doc = "Crop window size"]
pub mod cwsz;
#[doc = "DT (r) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt`]
module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "Data register"]
pub mod dt;
#[doc = "ACTRL (rw) register accessor: Advanced Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`actrl`]
module"]
pub type ACTRL = crate::Reg<actrl::ACTRL_SPEC>;
#[doc = "Advanced Control register"]
pub mod actrl;
#[doc = "HSCF (rw) register accessor: Horizontal scaling control flow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hscf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hscf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hscf`]
module"]
pub type HSCF = crate::Reg<hscf::HSCF_SPEC>;
#[doc = "Horizontal scaling control flow"]
pub mod hscf;
#[doc = "VSCF (rw) register accessor: Vertical scaling control flow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vscf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vscf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`vscf`]
module"]
pub type VSCF = crate::Reg<vscf::VSCF_SPEC>;
#[doc = "Vertical scaling control flow"]
pub mod vscf;
#[doc = "FRF (rw) register accessor: Frame rate flow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`frf`]
module"]
pub type FRF = crate::Reg<frf::FRF_SPEC>;
#[doc = "Frame rate flow"]
pub mod frf;
#[doc = "BTH (rw) register accessor: Binarization threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bth`]
module"]
pub type BTH = crate::Reg<bth::BTH_SPEC>;
#[doc = "Binarization threshold"]
pub mod bth;
