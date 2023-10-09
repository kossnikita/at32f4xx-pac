#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status register"]
    pub sts: STS,
    #[doc = "0x04 - Control register 1"]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - Control register 2"]
    pub ctrl2: CTRL2,
    #[doc = "0x0c - Compare 1"]
    pub c1: C1,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - Compare 2"]
    pub c2: C2,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - Compare 3"]
    pub c3: C3,
}
#[doc = "STS (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "Status register"]
pub mod sts;
#[doc = "CTRL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (r) register accessor: Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control register 2"]
pub mod ctrl2;
#[doc = "C1 (rw) register accessor: Compare 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1`]
module"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "Compare 1"]
pub mod c1;
#[doc = "C2 (rw) register accessor: Compare 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c2`]
module"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "Compare 2"]
pub mod c2;
#[doc = "C3 (rw) register accessor: Compare 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c3`]
module"]
pub type C3 = crate::Reg<c3::C3_SPEC>;
#[doc = "Compare 3"]
pub mod c3;
