#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    pub ept0: EPT0,
    #[doc = "0x04 - endpoint 1 register"]
    pub ept1: EPT1,
    #[doc = "0x08 - endpoint 2 register"]
    pub ept2: EPT2,
    #[doc = "0x0c - endpoint 3 register"]
    pub ept3: EPT3,
    #[doc = "0x10 - endpoint 4 register"]
    pub ept4: EPT4,
    #[doc = "0x14 - endpoint 5 register"]
    pub ept5: EPT5,
    #[doc = "0x18 - endpoint 6 register"]
    pub ept6: EPT6,
    #[doc = "0x1c - endpoint 7 register"]
    pub ept7: EPT7,
    _reserved8: [u8; 0x20],
    #[doc = "0x40 - control register"]
    pub ctrl: CTRL,
    #[doc = "0x44 - interrupt status register"]
    pub intsts: INTSTS,
    #[doc = "0x48 - frame number register"]
    pub sofrnum: SOFRNUM,
    #[doc = "0x4c - device address"]
    pub devaddr: DEVADDR,
    #[doc = "0x50 - Buffer table address"]
    pub buftbl: BUFTBL,
}
#[doc = "EPT0 (rw) register accessor: endpoint 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ept0`]
module"]
pub type EPT0 = crate::Reg<ept0::EPT0_SPEC>;
#[doc = "endpoint 0 register"]
pub mod ept0;
#[doc = "EPT1 (rw) register accessor: endpoint 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ept1`]
module"]
pub type EPT1 = crate::Reg<ept1::EPT1_SPEC>;
#[doc = "endpoint 1 register"]
pub mod ept1;
#[doc = "EPT2 (rw) register accessor: endpoint 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ept2`]
module"]
pub type EPT2 = crate::Reg<ept2::EPT2_SPEC>;
#[doc = "endpoint 2 register"]
pub mod ept2;
#[doc = "EPT3 (rw) register accessor: endpoint 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ept3`]
module"]
pub type EPT3 = crate::Reg<ept3::EPT3_SPEC>;
#[doc = "endpoint 3 register"]
pub mod ept3;
#[doc = "EPT4 (rw) register accessor: endpoint 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ept4`]
module"]
pub type EPT4 = crate::Reg<ept4::EPT4_SPEC>;
#[doc = "endpoint 4 register"]
pub mod ept4;
#[doc = "EPT5 (rw) register accessor: endpoint 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ept5`]
module"]
pub type EPT5 = crate::Reg<ept5::EPT5_SPEC>;
#[doc = "endpoint 5 register"]
pub mod ept5;
#[doc = "EPT6 (rw) register accessor: endpoint 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ept6`]
module"]
pub type EPT6 = crate::Reg<ept6::EPT6_SPEC>;
#[doc = "endpoint 6 register"]
pub mod ept6;
#[doc = "EPT7 (rw) register accessor: endpoint 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ept7`]
module"]
pub type EPT7 = crate::Reg<ept7::EPT7_SPEC>;
#[doc = "endpoint 7 register"]
pub mod ept7;
#[doc = "CTRL (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "control register"]
pub mod ctrl;
#[doc = "INTSTS (rw) register accessor: interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intsts`]
module"]
pub type INTSTS = crate::Reg<intsts::INTSTS_SPEC>;
#[doc = "interrupt status register"]
pub mod intsts;
#[doc = "SOFRNUM (rw) register accessor: frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sofrnum::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sofrnum::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sofrnum`]
module"]
pub type SOFRNUM = crate::Reg<sofrnum::SOFRNUM_SPEC>;
#[doc = "frame number register"]
pub mod sofrnum;
#[doc = "DEVADDR (rw) register accessor: device address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`devaddr`]
module"]
pub type DEVADDR = crate::Reg<devaddr::DEVADDR_SPEC>;
#[doc = "device address"]
pub mod devaddr;
#[doc = "BUFTBL (rw) register accessor: Buffer table address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buftbl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buftbl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`buftbl`]
module"]
pub type BUFTBL = crate::Reg<buftbl::BUFTBL_SPEC>;
#[doc = "Buffer table address"]
pub mod buftbl;
