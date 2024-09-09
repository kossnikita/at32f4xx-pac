#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    evtout: EVTOUT,
    remap: REMAP,
    exintc1: EXINTC1,
    exintc2: EXINTC2,
    exintc3: EXINTC3,
    exintc4: EXINTC4,
    _reserved6: [u8; 0x04],
    remap2: REMAP2,
    remap3: REMAP3,
    remap4: REMAP4,
    remap5: REMAP5,
    remap6: REMAP6,
    remap7: REMAP7,
    remap8: REMAP8,
}
impl RegisterBlock {
    #[doc = "0x00 - Event output register (IOMUX_EVTOUT)"]
    #[inline(always)]
    pub const fn evtout(&self) -> &EVTOUT {
        &self.evtout
    }
    #[doc = "0x04 - IO MUX remap register (IOMUX_REMAP)"]
    #[inline(always)]
    pub const fn remap(&self) -> &REMAP {
        &self.remap
    }
    #[doc = "0x08 - External interrupt configuration register 1 (IOMUX_EXINTC1)"]
    #[inline(always)]
    pub const fn exintc1(&self) -> &EXINTC1 {
        &self.exintc1
    }
    #[doc = "0x0c - External interrupt configuration register 2 (IOMUX_EXINTC2)"]
    #[inline(always)]
    pub const fn exintc2(&self) -> &EXINTC2 {
        &self.exintc2
    }
    #[doc = "0x10 - External interrupt configuration register 3 (IOMUX_EXINTC3)"]
    #[inline(always)]
    pub const fn exintc3(&self) -> &EXINTC3 {
        &self.exintc3
    }
    #[doc = "0x14 - External interrupt configuration register 4 (IOMUX_EXINTC4)"]
    #[inline(always)]
    pub const fn exintc4(&self) -> &EXINTC4 {
        &self.exintc4
    }
    #[doc = "0x1c - IO MUX remap register 2 (IOMUX_REMAP2)"]
    #[inline(always)]
    pub const fn remap2(&self) -> &REMAP2 {
        &self.remap2
    }
    #[doc = "0x20 - IO MUX remap register 3 (IOMUX_REMAP3)"]
    #[inline(always)]
    pub const fn remap3(&self) -> &REMAP3 {
        &self.remap3
    }
    #[doc = "0x24 - IO MUX remap register 4 (IOMUX_REMAP4)"]
    #[inline(always)]
    pub const fn remap4(&self) -> &REMAP4 {
        &self.remap4
    }
    #[doc = "0x28 - IO MUX remap register 5 (IOMUX_REMAP5)"]
    #[inline(always)]
    pub const fn remap5(&self) -> &REMAP5 {
        &self.remap5
    }
    #[doc = "0x2c - IO MUX remap register 6 (IOMUX_REMAP6)"]
    #[inline(always)]
    pub const fn remap6(&self) -> &REMAP6 {
        &self.remap6
    }
    #[doc = "0x30 - IO MUX remap register 7 (IOMUX_REMAP7)"]
    #[inline(always)]
    pub const fn remap7(&self) -> &REMAP7 {
        &self.remap7
    }
    #[doc = "0x34 - IO MUX remap register 8 (IOMUX_REMAP8)"]
    #[inline(always)]
    pub const fn remap8(&self) -> &REMAP8 {
        &self.remap8
    }
}
#[doc = "EVTOUT (rw) register accessor: Event output register (IOMUX_EVTOUT)\n\nYou can [`read`](crate::Reg::read) this register and get [`evtout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evtout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtout`]
module"]
pub type EVTOUT = crate::Reg<evtout::EVTOUT_SPEC>;
#[doc = "Event output register (IOMUX_EVTOUT)"]
pub mod evtout;
#[doc = "REMAP (rw) register accessor: IO MUX remap register (IOMUX_REMAP)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap`]
module"]
pub type REMAP = crate::Reg<remap::REMAP_SPEC>;
#[doc = "IO MUX remap register (IOMUX_REMAP)"]
pub mod remap;
#[doc = "EXINTC1 (rw) register accessor: External interrupt configuration register 1 (IOMUX_EXINTC1)\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc1`]
module"]
pub type EXINTC1 = crate::Reg<exintc1::EXINTC1_SPEC>;
#[doc = "External interrupt configuration register 1 (IOMUX_EXINTC1)"]
pub mod exintc1;
#[doc = "EXINTC2 (rw) register accessor: External interrupt configuration register 2 (IOMUX_EXINTC2)\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc2`]
module"]
pub type EXINTC2 = crate::Reg<exintc2::EXINTC2_SPEC>;
#[doc = "External interrupt configuration register 2 (IOMUX_EXINTC2)"]
pub mod exintc2;
#[doc = "EXINTC3 (rw) register accessor: External interrupt configuration register 3 (IOMUX_EXINTC3)\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc3`]
module"]
pub type EXINTC3 = crate::Reg<exintc3::EXINTC3_SPEC>;
#[doc = "External interrupt configuration register 3 (IOMUX_EXINTC3)"]
pub mod exintc3;
#[doc = "EXINTC4 (rw) register accessor: External interrupt configuration register 4 (IOMUX_EXINTC4)\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc4`]
module"]
pub type EXINTC4 = crate::Reg<exintc4::EXINTC4_SPEC>;
#[doc = "External interrupt configuration register 4 (IOMUX_EXINTC4)"]
pub mod exintc4;
#[doc = "REMAP2 (rw) register accessor: IO MUX remap register 2 (IOMUX_REMAP2)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap2`]
module"]
pub type REMAP2 = crate::Reg<remap2::REMAP2_SPEC>;
#[doc = "IO MUX remap register 2 (IOMUX_REMAP2)"]
pub mod remap2;
#[doc = "REMAP3 (rw) register accessor: IO MUX remap register 3 (IOMUX_REMAP3)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap3`]
module"]
pub type REMAP3 = crate::Reg<remap3::REMAP3_SPEC>;
#[doc = "IO MUX remap register 3 (IOMUX_REMAP3)"]
pub mod remap3;
#[doc = "REMAP4 (rw) register accessor: IO MUX remap register 4 (IOMUX_REMAP4)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap4`]
module"]
pub type REMAP4 = crate::Reg<remap4::REMAP4_SPEC>;
#[doc = "IO MUX remap register 4 (IOMUX_REMAP4)"]
pub mod remap4;
#[doc = "REMAP5 (rw) register accessor: IO MUX remap register 5 (IOMUX_REMAP5)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap5`]
module"]
pub type REMAP5 = crate::Reg<remap5::REMAP5_SPEC>;
#[doc = "IO MUX remap register 5 (IOMUX_REMAP5)"]
pub mod remap5;
#[doc = "REMAP6 (rw) register accessor: IO MUX remap register 6 (IOMUX_REMAP6)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap6`]
module"]
pub type REMAP6 = crate::Reg<remap6::REMAP6_SPEC>;
#[doc = "IO MUX remap register 6 (IOMUX_REMAP6)"]
pub mod remap6;
#[doc = "REMAP7 (rw) register accessor: IO MUX remap register 7 (IOMUX_REMAP7)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap7`]
module"]
pub type REMAP7 = crate::Reg<remap7::REMAP7_SPEC>;
#[doc = "IO MUX remap register 7 (IOMUX_REMAP7)"]
pub mod remap7;
#[doc = "REMAP8 (rw) register accessor: IO MUX remap register 8 (IOMUX_REMAP8)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap8`]
module"]
pub type REMAP8 = crate::Reg<remap8::REMAP8_SPEC>;
#[doc = "IO MUX remap register 8 (IOMUX_REMAP8)"]
pub mod remap8;
