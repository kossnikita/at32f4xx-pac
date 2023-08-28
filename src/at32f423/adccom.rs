#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Common control register"]
    pub cctrl: CCTRL,
}
#[doc = "CCTRL (rw) register accessor: Common control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cctrl`]
module"]
pub type CCTRL = crate::Reg<cctrl::CCTRL_SPEC>;
#[doc = "Common control register"]
pub mod cctrl;
