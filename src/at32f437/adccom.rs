#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Common status register"]
    pub csts: CSTS,
    #[doc = "0x04 - Common control register"]
    pub cctrl: CCTRL,
    #[doc = "0x08 - Common Ordinary data register"]
    pub codt: CODT,
}
#[doc = "CSTS (r) register accessor: Common status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`csts`]
module"]
pub type CSTS = crate::Reg<csts::CSTS_SPEC>;
#[doc = "Common status register"]
pub mod csts;
#[doc = "CCTRL (rw) register accessor: Common control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cctrl`]
module"]
pub type CCTRL = crate::Reg<cctrl::CCTRL_SPEC>;
#[doc = "Common control register"]
pub mod cctrl;
#[doc = "CODT (r) register accessor: Common Ordinary data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`codt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`codt`]
module"]
pub type CODT = crate::Reg<codt::CODT_SPEC>;
#[doc = "Common Ordinary data register"]
pub mod codt;
