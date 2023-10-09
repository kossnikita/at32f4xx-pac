#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event output register (IOMUX_EVTOUT)"]
    pub evtout: EVTOUT,
    #[doc = "0x04 - IO MUX remap register (IOMUX_REMAP)"]
    pub remap: REMAP,
    #[doc = "0x08 - External interrupt configuration register 1 (IOMUX_EXINTC1)"]
    pub exintc1: EXINTC1,
    #[doc = "0x0c - External interrupt configuration register 2 (IOMUX_EXINTC2)"]
    pub exintc2: EXINTC2,
    #[doc = "0x10 - External interrupt configuration register 3 (IOMUX_EXINTC3)"]
    pub exintc3: EXINTC3,
    #[doc = "0x14 - External interrupt configuration register 4 (IOMUX_EXINTC4)"]
    pub exintc4: EXINTC4,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - IO MUX remap register 2 (IOMUX_REMAP2)"]
    pub remap2: REMAP2,
}
#[doc = "EVTOUT (rw) register accessor: Event output register (IOMUX_EVTOUT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`evtout`]
module"]
pub type EVTOUT = crate::Reg<evtout::EVTOUT_SPEC>;
#[doc = "Event output register (IOMUX_EVTOUT)"]
pub mod evtout;
#[doc = "REMAP (rw) register accessor: IO MUX remap register (IOMUX_REMAP)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`remap`]
module"]
pub type REMAP = crate::Reg<remap::REMAP_SPEC>;
#[doc = "IO MUX remap register (IOMUX_REMAP)"]
pub mod remap;
#[doc = "EXINTC1 (rw) register accessor: External interrupt configuration register 1 (IOMUX_EXINTC1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exintc1`]
module"]
pub type EXINTC1 = crate::Reg<exintc1::EXINTC1_SPEC>;
#[doc = "External interrupt configuration register 1 (IOMUX_EXINTC1)"]
pub mod exintc1;
#[doc = "EXINTC2 (rw) register accessor: External interrupt configuration register 2 (IOMUX_EXINTC2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exintc2`]
module"]
pub type EXINTC2 = crate::Reg<exintc2::EXINTC2_SPEC>;
#[doc = "External interrupt configuration register 2 (IOMUX_EXINTC2)"]
pub mod exintc2;
#[doc = "EXINTC3 (rw) register accessor: External interrupt configuration register 3 (IOMUX_EXINTC3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exintc3`]
module"]
pub type EXINTC3 = crate::Reg<exintc3::EXINTC3_SPEC>;
#[doc = "External interrupt configuration register 3 (IOMUX_EXINTC3)"]
pub mod exintc3;
#[doc = "EXINTC4 (rw) register accessor: External interrupt configuration register 4 (IOMUX_EXINTC4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`exintc4`]
module"]
pub type EXINTC4 = crate::Reg<exintc4::EXINTC4_SPEC>;
#[doc = "External interrupt configuration register 4 (IOMUX_EXINTC4)"]
pub mod exintc4;
#[doc = "REMAP2 (rw) register accessor: IO MUX remap register 2 (IOMUX_REMAP2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`remap2`]
module"]
pub type REMAP2 = crate::Reg<remap2::REMAP2_SPEC>;
#[doc = "IO MUX remap register 2 (IOMUX_REMAP2)"]
pub mod remap2;
