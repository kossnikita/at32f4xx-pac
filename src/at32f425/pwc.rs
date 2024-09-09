#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    ctrlsts: CTRLSTS,
}
impl RegisterBlock {
    #[doc = "0x00 - Power control register (PWC_CTRL)"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Power control register (PWC_CTRLSTS)"]
    #[inline(always)]
    pub const fn ctrlsts(&self) -> &CTRLSTS {
        &self.ctrlsts
    }
}
#[doc = "CTRL (rw) register accessor: Power control register (PWC_CTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Power control register (PWC_CTRL)"]
pub mod ctrl;
#[doc = "CTRLSTS (rw) register accessor: Power control register (PWC_CTRLSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlsts`]
module"]
pub type CTRLSTS = crate::Reg<ctrlsts::CTRLSTS_SPEC>;
#[doc = "Power control register (PWC_CTRLSTS)"]
pub mod ctrlsts;
