#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - time register"]
    pub time: TIME,
    #[doc = "0x04 - date register"]
    pub date: DATE,
    #[doc = "0x08 - control register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - initialization and status register"]
    pub sts: STS,
    #[doc = "0x10 - Divider register"]
    pub div: DIV,
    #[doc = "0x14 - Wakeup timer register"]
    pub wat: WAT,
    _reserved6: [u8; 0x06],
    #[doc = "0x1c - Alarm A register"]
    pub ala: ALA,
    _reserved7: [u8; 0x04],
    #[doc = "0x24 - write protection register"]
    pub wp: WP,
    _reserved8: [u8; 0x03],
    #[doc = "0x28 - sub second register"]
    pub sbs: SBS,
    _reserved9: [u8; 0x02],
    #[doc = "0x2c - time adjust register"]
    pub tadj: TADJ,
    #[doc = "0x30 - time stamp time register"]
    pub tstm: TSTM,
    #[doc = "0x34 - timestamp date register"]
    pub tsdt: TSDT,
    #[doc = "0x38 - timestamp sub second register"]
    pub tssbs: TSSBS,
    #[doc = "0x3c - calibration register"]
    pub scal: SCAL,
    #[doc = "0x40 - tamper and alternate function configuration register"]
    pub tamp: TAMP,
    #[doc = "0x44 - alarm A sub second register"]
    pub alasbs: ALASBS,
    _reserved16: [u8; 0x08],
    #[doc = "0x50..0x64 - Battery powered domain register"]
    pub bprdt: [BPRDT; 5],
}
impl RegisterBlock {
    #[doc = "0x50 - Battery powered domain register"]
    #[inline(always)]
    pub fn bpr1dt(&self) -> &BPRDT {
        &self.bprdt[0]
    }
    #[doc = "0x54 - Battery powered domain register"]
    #[inline(always)]
    pub fn bpr2dt(&self) -> &BPRDT {
        &self.bprdt[1]
    }
    #[doc = "0x58 - Battery powered domain register"]
    #[inline(always)]
    pub fn bpr3dt(&self) -> &BPRDT {
        &self.bprdt[2]
    }
    #[doc = "0x5c - Battery powered domain register"]
    #[inline(always)]
    pub fn bpr4dt(&self) -> &BPRDT {
        &self.bprdt[3]
    }
    #[doc = "0x60 - Battery powered domain register"]
    #[inline(always)]
    pub fn bpr5dt(&self) -> &BPRDT {
        &self.bprdt[4]
    }
}
#[doc = "TIME (rw) register accessor: time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`time`]
module"]
pub type TIME = crate::Reg<time::TIME_SPEC>;
#[doc = "time register"]
pub mod time;
#[doc = "DATE (rw) register accessor: date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`]
module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "date register"]
pub mod date;
#[doc = "CTRL (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "control register"]
pub mod ctrl;
#[doc = "STS (rw) register accessor: initialization and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "initialization and status register"]
pub mod sts;
#[doc = "DIV (rw) register accessor: Divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`div`]
module"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Divider register"]
pub mod div;
#[doc = "WAT (rw) register accessor: Wakeup timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wat`]
module"]
pub type WAT = crate::Reg<wat::WAT_SPEC>;
#[doc = "Wakeup timer register"]
pub mod wat;
#[doc = "ALA (rw) register accessor: Alarm A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ala::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ala::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ala`]
module"]
pub type ALA = crate::Reg<ala::ALA_SPEC>;
#[doc = "Alarm A register"]
pub mod ala;
#[doc = "WP (w) register accessor: write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wp::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wp`]
module"]
pub type WP = crate::Reg<wp::WP_SPEC>;
#[doc = "write protection register"]
pub mod wp;
#[doc = "SBS (r) register accessor: sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sbs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sbs`]
module"]
pub type SBS = crate::Reg<sbs::SBS_SPEC>;
#[doc = "sub second register"]
pub mod sbs;
#[doc = "TADJ (w) register accessor: time adjust register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tadj::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tadj`]
module"]
pub type TADJ = crate::Reg<tadj::TADJ_SPEC>;
#[doc = "time adjust register"]
pub mod tadj;
#[doc = "TSTM (r) register accessor: time stamp time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tstm`]
module"]
pub type TSTM = crate::Reg<tstm::TSTM_SPEC>;
#[doc = "time stamp time register"]
pub mod tstm;
#[doc = "TSDT (r) register accessor: timestamp date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsdt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tsdt`]
module"]
pub type TSDT = crate::Reg<tsdt::TSDT_SPEC>;
#[doc = "timestamp date register"]
pub mod tsdt;
#[doc = "TSSBS (r) register accessor: timestamp sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tssbs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tssbs`]
module"]
pub type TSSBS = crate::Reg<tssbs::TSSBS_SPEC>;
#[doc = "timestamp sub second register"]
pub mod tssbs;
#[doc = "SCAL (rw) register accessor: calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`scal`]
module"]
pub type SCAL = crate::Reg<scal::SCAL_SPEC>;
#[doc = "calibration register"]
pub mod scal;
#[doc = "TAMP (rw) register accessor: tamper and alternate function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tamp`]
module"]
pub type TAMP = crate::Reg<tamp::TAMP_SPEC>;
#[doc = "tamper and alternate function configuration register"]
pub mod tamp;
#[doc = "ALASBS (rw) register accessor: alarm A sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alasbs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alasbs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`alasbs`]
module"]
pub type ALASBS = crate::Reg<alasbs::ALASBS_SPEC>;
#[doc = "alarm A sub second register"]
pub mod alasbs;
#[doc = "BPRDT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bprdt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bprdt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bprdt`]
module"]
pub type BPRDT = crate::Reg<bprdt::BPRDT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bprdt;
