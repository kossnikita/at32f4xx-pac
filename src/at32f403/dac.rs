#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register (DAC_CTRL)"]
    pub ctrl: CTRL,
    #[doc = "0x04 - DAC software trigger register(DAC_SWTRIGR)"]
    pub swtrg: SWTRG,
    #[doc = "0x08 - DAC1 12-bit right-aligned data holding register(DAC_D1DTH12R)"]
    pub d1dth12r: D1DTH12R,
    #[doc = "0x0c - DAC1 12-bit left aligned data holding register (DAC_D1DTH12L)"]
    pub d1dth12l: D1DTH12L,
    #[doc = "0x10 - DAC1 8-bit right aligned data holding register (DAC_D1DTH8R)"]
    pub d1dth8r: D1DTH8R,
    #[doc = "0x14 - DAC2 12-bit right aligned data holding register (DAC_D2DTH12R)"]
    pub d2dth12r: D2DTH12R,
    #[doc = "0x18 - DAC2 12-bit left aligned data holding register (DAC_D2DTH12L)"]
    pub d2dth12l: D2DTH12L,
    #[doc = "0x1c - DAC2 8-bit right-aligned data holding register (DAC_D2DTH8R)"]
    pub d2dth8r: D2DTH8R,
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register (DAC_DDTH12R), Bits 31:28 Reserved, Bits 15:12 Reserved"]
    pub ddth12r: DDTH12R,
    #[doc = "0x24 - DUAL DAC 12-bit left aligned data holding register (DAC_DDTH12L), Bits 19:16 Reserved, Bits 3:0 Reserved"]
    pub ddth12l: DDTH12L,
    #[doc = "0x28 - DUAL DAC 8-bit right aligned data holding register (DAC_DDTH8R), Bits 31:16 Reserved"]
    pub ddth8r: DDTH8R,
    #[doc = "0x2c - DAC1 data output register (DAC_D1ODT)"]
    pub d1odt: D1ODT,
    #[doc = "0x30 - DAC2 data output register (DAC_D2ODT)"]
    pub d2odt: D2ODT,
}
#[doc = "CTRL (rw) register accessor: Control register (DAC_CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register (DAC_CTRL)"]
pub mod ctrl;
#[doc = "SWTRG (rw) register accessor: DAC software trigger register(DAC_SWTRIGR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swtrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`swtrg`]
module"]
pub type SWTRG = crate::Reg<swtrg::SWTRG_SPEC>;
#[doc = "DAC software trigger register(DAC_SWTRIGR)"]
pub mod swtrg;
#[doc = "D1DTH12R (rw) register accessor: DAC1 12-bit right-aligned data holding register(DAC_D1DTH12R)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1dth12r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1dth12r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d1dth12r`]
module"]
pub type D1DTH12R = crate::Reg<d1dth12r::D1DTH12R_SPEC>;
#[doc = "DAC1 12-bit right-aligned data holding register(DAC_D1DTH12R)"]
pub mod d1dth12r;
#[doc = "D1DTH12L (rw) register accessor: DAC1 12-bit left aligned data holding register (DAC_D1DTH12L)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1dth12l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1dth12l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d1dth12l`]
module"]
pub type D1DTH12L = crate::Reg<d1dth12l::D1DTH12L_SPEC>;
#[doc = "DAC1 12-bit left aligned data holding register (DAC_D1DTH12L)"]
pub mod d1dth12l;
#[doc = "D1DTH8R (rw) register accessor: DAC1 8-bit right aligned data holding register (DAC_D1DTH8R)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1dth8r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1dth8r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d1dth8r`]
module"]
pub type D1DTH8R = crate::Reg<d1dth8r::D1DTH8R_SPEC>;
#[doc = "DAC1 8-bit right aligned data holding register (DAC_D1DTH8R)"]
pub mod d1dth8r;
#[doc = "D2DTH12R (rw) register accessor: DAC2 12-bit right aligned data holding register (DAC_D2DTH12R)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2dth12r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2dth12r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d2dth12r`]
module"]
pub type D2DTH12R = crate::Reg<d2dth12r::D2DTH12R_SPEC>;
#[doc = "DAC2 12-bit right aligned data holding register (DAC_D2DTH12R)"]
pub mod d2dth12r;
#[doc = "D2DTH12L (rw) register accessor: DAC2 12-bit left aligned data holding register (DAC_D2DTH12L)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2dth12l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2dth12l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d2dth12l`]
module"]
pub type D2DTH12L = crate::Reg<d2dth12l::D2DTH12L_SPEC>;
#[doc = "DAC2 12-bit left aligned data holding register (DAC_D2DTH12L)"]
pub mod d2dth12l;
#[doc = "D2DTH8R (rw) register accessor: DAC2 8-bit right-aligned data holding register (DAC_D2DTH8R)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2dth8r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2dth8r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d2dth8r`]
module"]
pub type D2DTH8R = crate::Reg<d2dth8r::D2DTH8R_SPEC>;
#[doc = "DAC2 8-bit right-aligned data holding register (DAC_D2DTH8R)"]
pub mod d2dth8r;
#[doc = "DDTH12R (rw) register accessor: Dual DAC 12-bit right-aligned data holding register (DAC_DDTH12R), Bits 31:28 Reserved, Bits 15:12 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddth12r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddth12r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ddth12r`]
module"]
pub type DDTH12R = crate::Reg<ddth12r::DDTH12R_SPEC>;
#[doc = "Dual DAC 12-bit right-aligned data holding register (DAC_DDTH12R), Bits 31:28 Reserved, Bits 15:12 Reserved"]
pub mod ddth12r;
#[doc = "DDTH12L (rw) register accessor: DUAL DAC 12-bit left aligned data holding register (DAC_DDTH12L), Bits 19:16 Reserved, Bits 3:0 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddth12l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddth12l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ddth12l`]
module"]
pub type DDTH12L = crate::Reg<ddth12l::DDTH12L_SPEC>;
#[doc = "DUAL DAC 12-bit left aligned data holding register (DAC_DDTH12L), Bits 19:16 Reserved, Bits 3:0 Reserved"]
pub mod ddth12l;
#[doc = "DDTH8R (rw) register accessor: DUAL DAC 8-bit right aligned data holding register (DAC_DDTH8R), Bits 31:16 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddth8r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddth8r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ddth8r`]
module"]
pub type DDTH8R = crate::Reg<ddth8r::DDTH8R_SPEC>;
#[doc = "DUAL DAC 8-bit right aligned data holding register (DAC_DDTH8R), Bits 31:16 Reserved"]
pub mod ddth8r;
#[doc = "D1ODT (r) register accessor: DAC1 data output register (DAC_D1ODT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1odt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d1odt`]
module"]
pub type D1ODT = crate::Reg<d1odt::D1ODT_SPEC>;
#[doc = "DAC1 data output register (DAC_D1ODT)"]
pub mod d1odt;
#[doc = "D2ODT (r) register accessor: DAC2 data output register (DAC_D2ODT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2odt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`d2odt`]
module"]
pub type D2ODT = crate::Reg<d2odt::D2ODT_SPEC>;
#[doc = "DAC2 data output register (DAC_D2ODT)"]
pub mod d2odt;
