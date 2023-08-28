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
    #[doc = "0x10 - Diveder register"]
    pub div: DIV,
    #[doc = "0x14 - Wakeup timer register"]
    pub wat: WAT,
    #[doc = "0x18 - Calibration register"]
    pub ccal: CCAL,
    #[doc = "0x1c - Alarm A register"]
    pub ala: ALA,
    #[doc = "0x20 - Alarm B register"]
    pub alb: ALB,
    #[doc = "0x24 - write protection register"]
    pub wp: WP,
    #[doc = "0x28 - sub second register"]
    pub sbs: SBS,
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
    #[doc = "0x48 - alarm B sub second register"]
    pub albsbs: ALBSBS,
    _reserved19: [u8; 0x04],
    #[doc = "0x50 - Battery powered domain register"]
    pub bpr1dt: BPR1DT,
    #[doc = "0x54 - Battery powered domain register"]
    pub bpr2dt: BPR2DT,
    #[doc = "0x58 - Battery powered domain register"]
    pub bpr3dt: BPR3DT,
    #[doc = "0x5c - Battery powered domain register"]
    pub bpr4dt: BPR4DT,
    #[doc = "0x60 - Battery powered domain register"]
    pub bpr5dt: BPR5DT,
    #[doc = "0x64 - Battery powered domain register"]
    pub bpr6dt: BPR6DT,
    #[doc = "0x68 - Battery powered domain register"]
    pub bpr7dt: BPR7DT,
    #[doc = "0x6c - Battery powered domain register"]
    pub bpr8dt: BPR8DT,
    #[doc = "0x70 - Battery powered domain register"]
    pub bpr9dt: BPR9DT,
    #[doc = "0x74 - Battery powered domain register"]
    pub bpr10dt: BPR10DT,
    #[doc = "0x78 - Battery powered domain register"]
    pub bpr11dt: BPR11DT,
    #[doc = "0x7c - Battery powered domain register"]
    pub bpr12dt: BPR12DT,
    #[doc = "0x80 - Battery powered domain register"]
    pub bpr13dt: BPR13DT,
    #[doc = "0x84 - Battery powered domain register"]
    pub bpr14dt: BPR14DT,
    #[doc = "0x88 - Battery powered domain register"]
    pub bpr15dt: BPR15DT,
    #[doc = "0x8c - Battery powered domain register"]
    pub bpr16dt: BPR16DT,
    #[doc = "0x90 - Battery powered domain register"]
    pub bpr17dt: BPR17DT,
    #[doc = "0x94 - Battery powered domain register"]
    pub bpr18dt: BPR18DT,
    #[doc = "0x98 - Battery powered domain register"]
    pub bpr19dt: BPR19DT,
    #[doc = "0x9c - Battery powered domain register"]
    pub bpr20dt: BPR20DT,
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
#[doc = "DIV (rw) register accessor: Diveder register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`div`]
module"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Diveder register"]
pub mod div;
#[doc = "WAT (rw) register accessor: Wakeup timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wat`]
module"]
pub type WAT = crate::Reg<wat::WAT_SPEC>;
#[doc = "Wakeup timer register"]
pub mod wat;
#[doc = "CCAL (rw) register accessor: Calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ccal`]
module"]
pub type CCAL = crate::Reg<ccal::CCAL_SPEC>;
#[doc = "Calibration register"]
pub mod ccal;
#[doc = "ALA (rw) register accessor: Alarm A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ala::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ala::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ala`]
module"]
pub type ALA = crate::Reg<ala::ALA_SPEC>;
#[doc = "Alarm A register"]
pub mod ala;
#[doc = "ALB (rw) register accessor: Alarm B register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`alb`]
module"]
pub type ALB = crate::Reg<alb::ALB_SPEC>;
#[doc = "Alarm B register"]
pub mod alb;
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
#[doc = "ALBSBS (rw) register accessor: alarm B sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`albsbs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`albsbs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`albsbs`]
module"]
pub type ALBSBS = crate::Reg<albsbs::ALBSBS_SPEC>;
#[doc = "alarm B sub second register"]
pub mod albsbs;
#[doc = "BPR1DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr1dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr1dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr1dt`]
module"]
pub type BPR1DT = crate::Reg<bpr1dt::BPR1DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr1dt;
#[doc = "BPR2DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr2dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr2dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr2dt`]
module"]
pub type BPR2DT = crate::Reg<bpr2dt::BPR2DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr2dt;
#[doc = "BPR3DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr3dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr3dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr3dt`]
module"]
pub type BPR3DT = crate::Reg<bpr3dt::BPR3DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr3dt;
#[doc = "BPR4DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr4dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr4dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr4dt`]
module"]
pub type BPR4DT = crate::Reg<bpr4dt::BPR4DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr4dt;
#[doc = "BPR5DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr5dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr5dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr5dt`]
module"]
pub type BPR5DT = crate::Reg<bpr5dt::BPR5DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr5dt;
#[doc = "BPR6DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr6dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr6dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr6dt`]
module"]
pub type BPR6DT = crate::Reg<bpr6dt::BPR6DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr6dt;
#[doc = "BPR7DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr7dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr7dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr7dt`]
module"]
pub type BPR7DT = crate::Reg<bpr7dt::BPR7DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr7dt;
#[doc = "BPR8DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr8dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr8dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr8dt`]
module"]
pub type BPR8DT = crate::Reg<bpr8dt::BPR8DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr8dt;
#[doc = "BPR9DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr9dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr9dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr9dt`]
module"]
pub type BPR9DT = crate::Reg<bpr9dt::BPR9DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr9dt;
#[doc = "BPR10DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr10dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr10dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr10dt`]
module"]
pub type BPR10DT = crate::Reg<bpr10dt::BPR10DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr10dt;
#[doc = "BPR11DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr11dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr11dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr11dt`]
module"]
pub type BPR11DT = crate::Reg<bpr11dt::BPR11DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr11dt;
#[doc = "BPR12DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr12dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr12dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr12dt`]
module"]
pub type BPR12DT = crate::Reg<bpr12dt::BPR12DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr12dt;
#[doc = "BPR13DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr13dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr13dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr13dt`]
module"]
pub type BPR13DT = crate::Reg<bpr13dt::BPR13DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr13dt;
#[doc = "BPR14DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr14dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr14dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr14dt`]
module"]
pub type BPR14DT = crate::Reg<bpr14dt::BPR14DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr14dt;
#[doc = "BPR15DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr15dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr15dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr15dt`]
module"]
pub type BPR15DT = crate::Reg<bpr15dt::BPR15DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr15dt;
#[doc = "BPR16DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr16dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr16dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr16dt`]
module"]
pub type BPR16DT = crate::Reg<bpr16dt::BPR16DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr16dt;
#[doc = "BPR17DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr17dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr17dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr17dt`]
module"]
pub type BPR17DT = crate::Reg<bpr17dt::BPR17DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr17dt;
#[doc = "BPR18DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr18dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr18dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr18dt`]
module"]
pub type BPR18DT = crate::Reg<bpr18dt::BPR18DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr18dt;
#[doc = "BPR19DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr19dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr19dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr19dt`]
module"]
pub type BPR19DT = crate::Reg<bpr19dt::BPR19DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr19dt;
#[doc = "BPR20DT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr20dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr20dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpr20dt`]
module"]
pub type BPR20DT = crate::Reg<bpr20dt::BPR20DT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bpr20dt;
