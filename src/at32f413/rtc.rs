#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrlh: CTRLH,
    ctrll: CTRLL,
    divh: DIVH,
    divl: DIVL,
    divcnth: DIVCNTH,
    divcntl: DIVCNTL,
    cnth: CNTH,
    cntl: CNTL,
    tah: TAH,
    tal: TAL,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC Control Register High"]
    #[inline(always)]
    pub const fn ctrlh(&self) -> &CTRLH {
        &self.ctrlh
    }
    #[doc = "0x04 - RTC Control Register Low"]
    #[inline(always)]
    pub const fn ctrll(&self) -> &CTRLL {
        &self.ctrll
    }
    #[doc = "0x08 - RTC Divider Register High"]
    #[inline(always)]
    pub const fn divh(&self) -> &DIVH {
        &self.divh
    }
    #[doc = "0x0c - RTC Divider Register Low"]
    #[inline(always)]
    pub const fn divl(&self) -> &DIVL {
        &self.divl
    }
    #[doc = "0x10 - RTC Divider Register High"]
    #[inline(always)]
    pub const fn divcnth(&self) -> &DIVCNTH {
        &self.divcnth
    }
    #[doc = "0x14 - RTC Divider Register Low"]
    #[inline(always)]
    pub const fn divcntl(&self) -> &DIVCNTL {
        &self.divcntl
    }
    #[doc = "0x18 - RTC Counter Register High"]
    #[inline(always)]
    pub const fn cnth(&self) -> &CNTH {
        &self.cnth
    }
    #[doc = "0x1c - RTC Counter Register Low"]
    #[inline(always)]
    pub const fn cntl(&self) -> &CNTL {
        &self.cntl
    }
    #[doc = "0x20 - RTC Alarm Register High"]
    #[inline(always)]
    pub const fn tah(&self) -> &TAH {
        &self.tah
    }
    #[doc = "0x24 - Time alarm register low"]
    #[inline(always)]
    pub const fn tal(&self) -> &TAL {
        &self.tal
    }
}
#[doc = "CTRLH (rw) register accessor: RTC Control Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlh`] module"]
pub type CTRLH = crate::Reg<ctrlh::CTRLH_SPEC>;
#[doc = "RTC Control Register High"]
pub mod ctrlh;
#[doc = "CTRLL (rw) register accessor: RTC Control Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrll`] module"]
pub type CTRLL = crate::Reg<ctrll::CTRLL_SPEC>;
#[doc = "RTC Control Register Low"]
pub mod ctrll;
#[doc = "DIVH (w) register accessor: RTC Divider Register High\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divh`] module"]
pub type DIVH = crate::Reg<divh::DIVH_SPEC>;
#[doc = "RTC Divider Register High"]
pub mod divh;
#[doc = "DIVL (w) register accessor: RTC Divider Register Low\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divl`] module"]
pub type DIVL = crate::Reg<divl::DIVL_SPEC>;
#[doc = "RTC Divider Register Low"]
pub mod divl;
#[doc = "DIVCNTH (rw) register accessor: RTC Divider Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`divcnth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divcnth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divcnth`] module"]
pub type DIVCNTH = crate::Reg<divcnth::DIVCNTH_SPEC>;
#[doc = "RTC Divider Register High"]
pub mod divcnth;
#[doc = "DIVCNTL (rw) register accessor: RTC Divider Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`divcntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divcntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divcntl`] module"]
pub type DIVCNTL = crate::Reg<divcntl::DIVCNTL_SPEC>;
#[doc = "RTC Divider Register Low"]
pub mod divcntl;
#[doc = "CNTH (rw) register accessor: RTC Counter Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`cnth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnth`] module"]
pub type CNTH = crate::Reg<cnth::CNTH_SPEC>;
#[doc = "RTC Counter Register High"]
pub mod cnth;
#[doc = "CNTL (rw) register accessor: RTC Counter Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl`] module"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "RTC Counter Register Low"]
pub mod cntl;
#[doc = "TAH (w) register accessor: RTC Alarm Register High\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tah::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tah`] module"]
pub type TAH = crate::Reg<tah::TAH_SPEC>;
#[doc = "RTC Alarm Register High"]
pub mod tah;
#[doc = "TAL (w) register accessor: Time alarm register low\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tal::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tal`] module"]
pub type TAL = crate::Reg<tal::TAL_SPEC>;
#[doc = "Time alarm register low"]
pub mod tal;
