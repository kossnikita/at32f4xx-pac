#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csts: CSTS,
    cctrl: CCTRL,
    codt: CODT,
}
impl RegisterBlock {
    #[doc = "0x00 - Common status register"]
    #[inline(always)]
    pub const fn csts(&self) -> &CSTS {
        &self.csts
    }
    #[doc = "0x04 - Common control register"]
    #[inline(always)]
    pub const fn cctrl(&self) -> &CCTRL {
        &self.cctrl
    }
    #[doc = "0x08 - Common Ordinary data register"]
    #[inline(always)]
    pub const fn codt(&self) -> &CODT {
        &self.codt
    }
}
#[doc = "CSTS (r) register accessor: Common status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csts`] module"]
pub type CSTS = crate::Reg<csts::CSTS_SPEC>;
#[doc = "Common status register"]
pub mod csts;
#[doc = "CCTRL (rw) register accessor: Common control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cctrl`] module"]
pub type CCTRL = crate::Reg<cctrl::CCTRL_SPEC>;
#[doc = "Common control register"]
pub mod cctrl;
#[doc = "CODT (r) register accessor: Common Ordinary data register\n\nYou can [`read`](crate::Reg::read) this register and get [`codt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codt`] module"]
pub type CODT = crate::Reg<codt::CODT_SPEC>;
#[doc = "Common Ordinary data register"]
pub mod codt;
