#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMP control/status register1"]
    pub ctrlsts1: CTRLSTS1,
    #[doc = "0x04 - CMP control/status register2"]
    pub ctrlsts2: CTRLSTS2,
}
#[doc = "CTRLSTS1 (rw) register accessor: CMP control/status register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrlsts1`]
module"]
pub type CTRLSTS1 = crate::Reg<ctrlsts1::CTRLSTS1_SPEC>;
#[doc = "CMP control/status register1"]
pub mod ctrlsts1;
#[doc = "CTRLSTS2 (rw) register accessor: CMP control/status register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrlsts2`]
module"]
pub type CTRLSTS2 = crate::Reg<ctrlsts2::CTRLSTS2_SPEC>;
#[doc = "CMP control/status register2"]
pub mod ctrlsts2;
