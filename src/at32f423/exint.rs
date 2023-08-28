#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x04 - Event enable register"]
    pub evten: EVTEN,
    #[doc = "0x08 - Rising polarity configuration register"]
    pub polcfg1: POLCFG1,
    #[doc = "0x0c - Falling polarity configuration register"]
    pub polcfg2: POLCFG2,
    #[doc = "0x10 - Software triggle register"]
    pub swtrg: SWTRG,
    #[doc = "0x14 - Interrupt status register"]
    pub intsts: INTSTS,
}
#[doc = "INTEN (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "EVTEN (rw) register accessor: Event enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`evten`]
module"]
pub type EVTEN = crate::Reg<evten::EVTEN_SPEC>;
#[doc = "Event enable register"]
pub mod evten;
#[doc = "POLCFG1 (rw) register accessor: Rising polarity configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`polcfg1`]
module"]
pub type POLCFG1 = crate::Reg<polcfg1::POLCFG1_SPEC>;
#[doc = "Rising polarity configuration register"]
pub mod polcfg1;
#[doc = "POLCFG2 (rw) register accessor: Falling polarity configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polcfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`polcfg2`]
module"]
pub type POLCFG2 = crate::Reg<polcfg2::POLCFG2_SPEC>;
#[doc = "Falling polarity configuration register"]
pub mod polcfg2;
#[doc = "SWTRG (rw) register accessor: Software triggle register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swtrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swtrg`]
module"]
pub type SWTRG = crate::Reg<swtrg::SWTRG_SPEC>;
#[doc = "Software triggle register"]
pub mod swtrg;
#[doc = "INTSTS (rw) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intsts`]
module"]
pub type INTSTS = crate::Reg<intsts::INTSTS_SPEC>;
#[doc = "Interrupt status register"]
pub mod intsts;
