#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Clock configuration register"]
    pub cfg: CFG,
    #[doc = "0x08 - Clock interrupt register"]
    pub clkint: CLKINT,
    #[doc = "0x0c - APB2 peripheral reset register"]
    pub apb2rst: APB2RST,
    #[doc = "0x10 - APB1 peripheral reset register"]
    pub apb1rst: APB1RST,
    #[doc = "0x14 - AHB Peripheral Clock enable register"]
    pub ahben: AHBEN,
    #[doc = "0x18 - APB2 peripheral clock enable register"]
    pub apb2en: APB2EN,
    #[doc = "0x1c - APB1 peripheral clock enable register"]
    pub apb1en: APB1EN,
    #[doc = "0x20 - Battery powered domain control register"]
    pub bpdc: BPDC,
    #[doc = "0x24 - Control/status register"]
    pub ctrlsts: CTRLSTS,
    _reserved10: [u8; 0x08],
    #[doc = "0x30 - Miscellaneous register 1"]
    pub misc1: MISC1,
    _reserved11: [u8; 0x1c],
    #[doc = "0x50 - Miscellaneous register 2"]
    pub misc2: MISC2,
    #[doc = "0x54 - Miscellaneous register 3"]
    pub misc3: MISC3,
    _reserved13: [u8; 0x04],
    #[doc = "0x5c - Interrupt remap register"]
    pub intmap: INTMAP,
}
#[doc = "CTRL (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Clock control register"]
pub mod ctrl;
#[doc = "CFG (rw) register accessor: Clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Clock configuration register"]
pub mod cfg;
#[doc = "CLKINT (rw) register accessor: Clock interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkint`]
module"]
pub type CLKINT = crate::Reg<clkint::CLKINT_SPEC>;
#[doc = "Clock interrupt register"]
pub mod clkint;
#[doc = "APB2RST (rw) register accessor: APB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb2rst`]
module"]
pub type APB2RST = crate::Reg<apb2rst::APB2RST_SPEC>;
#[doc = "APB2 peripheral reset register"]
pub mod apb2rst;
#[doc = "APB1RST (rw) register accessor: APB1 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb1rst`]
module"]
pub type APB1RST = crate::Reg<apb1rst::APB1RST_SPEC>;
#[doc = "APB1 peripheral reset register"]
pub mod apb1rst;
#[doc = "AHBEN (rw) register accessor: AHB Peripheral Clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahben`]
module"]
pub type AHBEN = crate::Reg<ahben::AHBEN_SPEC>;
#[doc = "AHB Peripheral Clock enable register"]
pub mod ahben;
#[doc = "APB2EN (rw) register accessor: APB2 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb2en`]
module"]
pub type APB2EN = crate::Reg<apb2en::APB2EN_SPEC>;
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2en;
#[doc = "APB1EN (rw) register accessor: APB1 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb1en`]
module"]
pub type APB1EN = crate::Reg<apb1en::APB1EN_SPEC>;
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1en;
#[doc = "BPDC (rw) register accessor: Battery powered domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpdc`]
module"]
pub type BPDC = crate::Reg<bpdc::BPDC_SPEC>;
#[doc = "Battery powered domain control register"]
pub mod bpdc;
#[doc = "CTRLSTS (rw) register accessor: Control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrlsts`]
module"]
pub type CTRLSTS = crate::Reg<ctrlsts::CTRLSTS_SPEC>;
#[doc = "Control/status register"]
pub mod ctrlsts;
#[doc = "MISC1 (rw) register accessor: Miscellaneous register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`misc1`]
module"]
pub type MISC1 = crate::Reg<misc1::MISC1_SPEC>;
#[doc = "Miscellaneous register 1"]
pub mod misc1;
#[doc = "MISC2 (rw) register accessor: Miscellaneous register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`misc2`]
module"]
pub type MISC2 = crate::Reg<misc2::MISC2_SPEC>;
#[doc = "Miscellaneous register 2"]
pub mod misc2;
#[doc = "MISC3 (rw) register accessor: Miscellaneous register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`misc3`]
module"]
pub type MISC3 = crate::Reg<misc3::MISC3_SPEC>;
#[doc = "Miscellaneous register 3"]
pub mod misc3;
#[doc = "INTMAP (rw) register accessor: Interrupt remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intmap`]
module"]
pub type INTMAP = crate::Reg<intmap::INTMAP_SPEC>;
#[doc = "Interrupt remap register"]
pub mod intmap;
