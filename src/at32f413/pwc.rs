#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Power control and status register"]
    pub ctrlsts: CTRLSTS,
}
#[doc = "CTRL (rw) register accessor: Power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Power control register"]
pub mod ctrl;
#[doc = "CTRLSTS (rw) register accessor: Power control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrlsts`]
module"]
pub type CTRLSTS = crate::Reg<ctrlsts::CTRLSTS_SPEC>;
#[doc = "Power control and status register"]
pub mod ctrlsts;
