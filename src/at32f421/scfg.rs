#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - configuration register 1"]
    pub cfg1: CFG1,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub exintc1: EXINTC1,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub exintc2: EXINTC2,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub exintc3: EXINTC3,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub exintc4: EXINTC4,
}
#[doc = "CFG1 (rw) register accessor: configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cfg1`]
module"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "configuration register 1"]
pub mod cfg1;
#[doc = "EXINTC1 (rw) register accessor: external interrupt configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exintc1`]
module"]
pub type EXINTC1 = crate::Reg<exintc1::EXINTC1_SPEC>;
#[doc = "external interrupt configuration register 1"]
pub mod exintc1;
#[doc = "EXINTC2 (rw) register accessor: external interrupt configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exintc2`]
module"]
pub type EXINTC2 = crate::Reg<exintc2::EXINTC2_SPEC>;
#[doc = "external interrupt configuration register 2"]
pub mod exintc2;
#[doc = "EXINTC3 (rw) register accessor: external interrupt configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exintc3`]
module"]
pub type EXINTC3 = crate::Reg<exintc3::EXINTC3_SPEC>;
#[doc = "external interrupt configuration register 3"]
pub mod exintc3;
#[doc = "EXINTC4 (rw) register accessor: external interrupt configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exintc4`]
module"]
pub type EXINTC4 = crate::Reg<exintc4::EXINTC4_SPEC>;
#[doc = "external interrupt configuration register 4"]
pub mod exintc4;
