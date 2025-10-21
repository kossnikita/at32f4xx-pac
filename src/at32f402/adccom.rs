#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    cctrl: CCTRL,
}
impl RegisterBlock {
    #[doc = "0x04 - Common control register"]
    #[inline(always)]
    pub const fn cctrl(&self) -> &CCTRL {
        &self.cctrl
    }
}
#[doc = "CCTRL (rw) register accessor: Common control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cctrl`] module"]
pub type CCTRL = crate::Reg<cctrl::CCTRL_SPEC>;
#[doc = "Common control register"]
pub mod cctrl;
