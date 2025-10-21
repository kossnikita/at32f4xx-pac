#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    time: TIME,
    date: DATE,
    ctrl: CTRL,
    sts: STS,
    div: DIV,
    wat: WAT,
    _reserved6: [u8; 0x06],
    ala: ALA,
    _reserved7: [u8; 0x04],
    wp: WP,
    _reserved8: [u8; 0x03],
    sbs: SBS,
    _reserved9: [u8; 0x02],
    tadj: TADJ,
    tstm: TSTM,
    tsdt: TSDT,
    tssbs: TSSBS,
    scal: SCAL,
    tamp: TAMP,
    alasbs: ALASBS,
    _reserved16: [u8; 0x08],
    bprdt: [BPRDT; 5],
}
impl RegisterBlock {
    #[doc = "0x00 - time register"]
    #[inline(always)]
    pub const fn time(&self) -> &TIME {
        &self.time
    }
    #[doc = "0x04 - date register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x08 - control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x0c - initialization and status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &STS {
        &self.sts
    }
    #[doc = "0x10 - Divider register"]
    #[inline(always)]
    pub const fn div(&self) -> &DIV {
        &self.div
    }
    #[doc = "0x14 - Wakeup timer register"]
    #[inline(always)]
    pub const fn wat(&self) -> &WAT {
        &self.wat
    }
    #[doc = "0x1c - Alarm A register"]
    #[inline(always)]
    pub const fn ala(&self) -> &ALA {
        &self.ala
    }
    #[doc = "0x24 - write protection register"]
    #[inline(always)]
    pub const fn wp(&self) -> &WP {
        &self.wp
    }
    #[doc = "0x28 - sub second register"]
    #[inline(always)]
    pub const fn sbs(&self) -> &SBS {
        &self.sbs
    }
    #[doc = "0x2c - time adjust register"]
    #[inline(always)]
    pub const fn tadj(&self) -> &TADJ {
        &self.tadj
    }
    #[doc = "0x30 - time stamp time register"]
    #[inline(always)]
    pub const fn tstm(&self) -> &TSTM {
        &self.tstm
    }
    #[doc = "0x34 - timestamp date register"]
    #[inline(always)]
    pub const fn tsdt(&self) -> &TSDT {
        &self.tsdt
    }
    #[doc = "0x38 - timestamp sub second register"]
    #[inline(always)]
    pub const fn tssbs(&self) -> &TSSBS {
        &self.tssbs
    }
    #[doc = "0x3c - calibration register"]
    #[inline(always)]
    pub const fn scal(&self) -> &SCAL {
        &self.scal
    }
    #[doc = "0x40 - tamper and alternate function configuration register"]
    #[inline(always)]
    pub const fn tamp(&self) -> &TAMP {
        &self.tamp
    }
    #[doc = "0x44 - alarm A sub second register"]
    #[inline(always)]
    pub const fn alasbs(&self) -> &ALASBS {
        &self.alasbs
    }
    #[doc = "0x50..0x64 - Battery powered domain register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `BPR1DT` register.</div>"]
    #[inline(always)]
    pub const fn bprdt(&self, n: usize) -> &BPRDT {
        &self.bprdt[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x64 - Battery powered domain register"]
    #[inline(always)]
    pub fn bprdt_iter(&self) -> impl Iterator<Item = &BPRDT> {
        self.bprdt.iter()
    }
    #[doc = "0x50 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr1dt(&self) -> &BPRDT {
        self.bprdt(0)
    }
    #[doc = "0x54 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr2dt(&self) -> &BPRDT {
        self.bprdt(1)
    }
    #[doc = "0x58 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr3dt(&self) -> &BPRDT {
        self.bprdt(2)
    }
    #[doc = "0x5c - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr4dt(&self) -> &BPRDT {
        self.bprdt(3)
    }
    #[doc = "0x60 - Battery powered domain register"]
    #[inline(always)]
    pub const fn bpr5dt(&self) -> &BPRDT {
        self.bprdt(4)
    }
}
#[doc = "TIME (rw) register accessor: time register\n\nYou can [`read`](crate::Reg::read) this register and get [`time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`] module"]
pub type TIME = crate::Reg<time::TIME_SPEC>;
#[doc = "time register"]
pub mod time;
#[doc = "DATE (rw) register accessor: date register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "date register"]
pub mod date;
#[doc = "CTRL (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "control register"]
pub mod ctrl;
#[doc = "STS (rw) register accessor: initialization and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`] module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "initialization and status register"]
pub mod sts;
#[doc = "DIV (rw) register accessor: Divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`] module"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Divider register"]
pub mod div;
#[doc = "WAT (rw) register accessor: Wakeup timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`wat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wat`] module"]
pub type WAT = crate::Reg<wat::WAT_SPEC>;
#[doc = "Wakeup timer register"]
pub mod wat;
#[doc = "ALA (rw) register accessor: Alarm A register\n\nYou can [`read`](crate::Reg::read) this register and get [`ala::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ala::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ala`] module"]
pub type ALA = crate::Reg<ala::ALA_SPEC>;
#[doc = "Alarm A register"]
pub mod ala;
#[doc = "WP (w) register accessor: write protection register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wp`] module"]
pub type WP = crate::Reg<wp::WP_SPEC>;
#[doc = "write protection register"]
pub mod wp;
#[doc = "SBS (r) register accessor: sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`sbs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbs`] module"]
pub type SBS = crate::Reg<sbs::SBS_SPEC>;
#[doc = "sub second register"]
pub mod sbs;
#[doc = "TADJ (w) register accessor: time adjust register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tadj::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tadj`] module"]
pub type TADJ = crate::Reg<tadj::TADJ_SPEC>;
#[doc = "time adjust register"]
pub mod tadj;
#[doc = "TSTM (r) register accessor: time stamp time register\n\nYou can [`read`](crate::Reg::read) this register and get [`tstm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstm`] module"]
pub type TSTM = crate::Reg<tstm::TSTM_SPEC>;
#[doc = "time stamp time register"]
pub mod tstm;
#[doc = "TSDT (r) register accessor: timestamp date register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsdt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsdt`] module"]
pub type TSDT = crate::Reg<tsdt::TSDT_SPEC>;
#[doc = "timestamp date register"]
pub mod tsdt;
#[doc = "TSSBS (r) register accessor: timestamp sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`tssbs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tssbs`] module"]
pub type TSSBS = crate::Reg<tssbs::TSSBS_SPEC>;
#[doc = "timestamp sub second register"]
pub mod tssbs;
#[doc = "SCAL (rw) register accessor: calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`scal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scal`] module"]
pub type SCAL = crate::Reg<scal::SCAL_SPEC>;
#[doc = "calibration register"]
pub mod scal;
#[doc = "TAMP (rw) register accessor: tamper and alternate function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tamp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp`] module"]
pub type TAMP = crate::Reg<tamp::TAMP_SPEC>;
#[doc = "tamper and alternate function configuration register"]
pub mod tamp;
#[doc = "ALASBS (rw) register accessor: alarm A sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`alasbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alasbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alasbs`] module"]
pub type ALASBS = crate::Reg<alasbs::ALASBS_SPEC>;
#[doc = "alarm A sub second register"]
pub mod alasbs;
#[doc = "BPRDT (rw) register accessor: Battery powered domain register\n\nYou can [`read`](crate::Reg::read) this register and get [`bprdt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bprdt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bprdt`] module"]
pub type BPRDT = crate::Reg<bprdt::BPRDT_SPEC>;
#[doc = "Battery powered domain register"]
pub mod bprdt;
