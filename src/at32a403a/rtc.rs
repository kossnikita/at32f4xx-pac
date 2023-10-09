#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Control Register High"]
    pub ctrlh: CTRLH,
    #[doc = "0x04 - RTC Control Register Low"]
    pub ctrll: CTRLL,
    #[doc = "0x08 - RTC Divider Register High"]
    pub divh: DIVH,
    #[doc = "0x0c - RTC Divider Register Low"]
    pub divl: DIVL,
    #[doc = "0x10 - RTC Divider Register High"]
    pub divcnth: DIVCNTH,
    #[doc = "0x14 - RTC Divider Register Low"]
    pub divcntl: DIVCNTL,
    #[doc = "0x18 - RTC Counter Register High"]
    pub cnth: CNTH,
    #[doc = "0x1c - RTC Counter Register Low"]
    pub cntl: CNTL,
    #[doc = "0x20 - RTC Alarm Register High"]
    pub tah: TAH,
    #[doc = "0x24 - Time alarm register low"]
    pub tal: TAL,
}
#[doc = "CTRLH (rw) register accessor: RTC Control Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrlh`]
module"]
pub type CTRLH = crate::Reg<ctrlh::CTRLH_SPEC>;
#[doc = "RTC Control Register High"]
pub mod ctrlh;
#[doc = "CTRLL (rw) register accessor: RTC Control Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrll`]
module"]
pub type CTRLL = crate::Reg<ctrll::CTRLL_SPEC>;
#[doc = "RTC Control Register Low"]
pub mod ctrll;
#[doc = "DIVH (w) register accessor: RTC Divider Register High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`divh`]
module"]
pub type DIVH = crate::Reg<divh::DIVH_SPEC>;
#[doc = "RTC Divider Register High"]
pub mod divh;
#[doc = "DIVL (w) register accessor: RTC Divider Register Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`divl`]
module"]
pub type DIVL = crate::Reg<divl::DIVL_SPEC>;
#[doc = "RTC Divider Register Low"]
pub mod divl;
#[doc = "DIVCNTH (rw) register accessor: RTC Divider Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divcnth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divcnth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`divcnth`]
module"]
pub type DIVCNTH = crate::Reg<divcnth::DIVCNTH_SPEC>;
#[doc = "RTC Divider Register High"]
pub mod divcnth;
#[doc = "DIVCNTL (rw) register accessor: RTC Divider Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divcntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divcntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`divcntl`]
module"]
pub type DIVCNTL = crate::Reg<divcntl::DIVCNTL_SPEC>;
#[doc = "RTC Divider Register Low"]
pub mod divcntl;
#[doc = "CNTH (rw) register accessor: RTC Counter Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cnth`]
module"]
pub type CNTH = crate::Reg<cnth::CNTH_SPEC>;
#[doc = "RTC Counter Register High"]
pub mod cnth;
#[doc = "CNTL (rw) register accessor: RTC Counter Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cntl`]
module"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "RTC Counter Register Low"]
pub mod cntl;
#[doc = "TAH (w) register accessor: RTC Alarm Register High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tah::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tah`]
module"]
pub type TAH = crate::Reg<tah::TAH_SPEC>;
#[doc = "RTC Alarm Register High"]
pub mod tah;
#[doc = "TAL (w) register accessor: Time alarm register low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tal::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tal`]
module"]
pub type TAL = crate::Reg<tal::TAL_SPEC>;
#[doc = "Time alarm register low"]
pub mod tal;
