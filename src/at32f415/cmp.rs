#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrlsts1: CTRLSTS1,
    ctrlsts2: CTRLSTS2,
}
impl RegisterBlock {
    #[doc = "0x00 - CMP control/status register1"]
    #[inline(always)]
    pub const fn ctrlsts1(&self) -> &CTRLSTS1 {
        &self.ctrlsts1
    }
    #[doc = "0x04 - CMP control/status register2"]
    #[inline(always)]
    pub const fn ctrlsts2(&self) -> &CTRLSTS2 {
        &self.ctrlsts2
    }
}
#[doc = "CTRLSTS1 (rw) register accessor: CMP control/status register1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlsts1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlsts1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlsts1`] module"]
pub type CTRLSTS1 = crate::Reg<ctrlsts1::CTRLSTS1_SPEC>;
#[doc = "CMP control/status register1"]
pub mod ctrlsts1;
#[doc = "CTRLSTS2 (rw) register accessor: CMP control/status register2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlsts2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlsts2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlsts2`] module"]
pub type CTRLSTS2 = crate::Reg<ctrlsts2::CTRLSTS2_SPEC>;
#[doc = "CMP control/status register2"]
pub mod ctrlsts2;
