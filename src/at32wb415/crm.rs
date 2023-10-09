#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Clock configuration register (CRM_CFG)"]
    pub cfg: CFG,
    #[doc = "0x08 - Clock interrupt register (CRM_CLKINT)"]
    pub clkint: CLKINT,
    #[doc = "0x0c - APB2 peripheral reset register (CRM_APB2RST)"]
    pub apb2rst: APB2RST,
    #[doc = "0x10 - APB1 peripheral reset register (CRM_APB1RST)"]
    pub apb1rst: APB1RST,
    #[doc = "0x14 - AHB Peripheral Clock enable register (CRM_AHBEN)"]
    pub ahben: AHBEN,
    #[doc = "0x18 - APB2 peripheral clock enable register (CRM_APB2EN)"]
    pub apb2en: APB2EN,
    #[doc = "0x1c - APB1 peripheral clock enable register (CRM_APB1EN)"]
    pub apb1en: APB1EN,
    #[doc = "0x20 - Battery powered domain control register (CRM_BPDC)"]
    pub bpdc: BPDC,
    #[doc = "0x24 - Control/status register (CRM_CTRLSTS)"]
    pub ctrlsts: CTRLSTS,
    #[doc = "0x28 - AHB reset register"]
    pub ahbrst: AHBRST,
    #[doc = "0x2c - PLL configuration register (RCC_PLL)"]
    pub pll: PLL,
    #[doc = "0x30 - Miscellaneous register1"]
    pub misc1: MISC1,
    _reserved13: [u8; 0x10],
    #[doc = "0x44 - OTGFS external ctrl register1"]
    pub otg_extctrl: OTG_EXTCTRL,
    _reserved14: [u8; 0x0c],
    #[doc = "0x54 - Miscellaneous register2"]
    pub misc2: MISC2,
}
#[doc = "CTRL (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Clock control register"]
pub mod ctrl;
#[doc = "CFG (rw) register accessor: Clock configuration register (CRM_CFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Clock configuration register (CRM_CFG)"]
pub mod cfg;
#[doc = "CLKINT (rw) register accessor: Clock interrupt register (CRM_CLKINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkint`]
module"]
pub type CLKINT = crate::Reg<clkint::CLKINT_SPEC>;
#[doc = "Clock interrupt register (CRM_CLKINT)"]
pub mod clkint;
#[doc = "APB2RST (rw) register accessor: APB2 peripheral reset register (CRM_APB2RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb2rst`]
module"]
pub type APB2RST = crate::Reg<apb2rst::APB2RST_SPEC>;
#[doc = "APB2 peripheral reset register (CRM_APB2RST)"]
pub mod apb2rst;
#[doc = "APB1RST (rw) register accessor: APB1 peripheral reset register (CRM_APB1RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb1rst`]
module"]
pub type APB1RST = crate::Reg<apb1rst::APB1RST_SPEC>;
#[doc = "APB1 peripheral reset register (CRM_APB1RST)"]
pub mod apb1rst;
#[doc = "AHBEN (rw) register accessor: AHB Peripheral Clock enable register (CRM_AHBEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahben`]
module"]
pub type AHBEN = crate::Reg<ahben::AHBEN_SPEC>;
#[doc = "AHB Peripheral Clock enable register (CRM_AHBEN)"]
pub mod ahben;
#[doc = "APB2EN (rw) register accessor: APB2 peripheral clock enable register (CRM_APB2EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb2en`]
module"]
pub type APB2EN = crate::Reg<apb2en::APB2EN_SPEC>;
#[doc = "APB2 peripheral clock enable register (CRM_APB2EN)"]
pub mod apb2en;
#[doc = "APB1EN (rw) register accessor: APB1 peripheral clock enable register (CRM_APB1EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb1en`]
module"]
pub type APB1EN = crate::Reg<apb1en::APB1EN_SPEC>;
#[doc = "APB1 peripheral clock enable register (CRM_APB1EN)"]
pub mod apb1en;
#[doc = "BPDC (rw) register accessor: Battery powered domain control register (CRM_BPDC)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bpdc`]
module"]
pub type BPDC = crate::Reg<bpdc::BPDC_SPEC>;
#[doc = "Battery powered domain control register (CRM_BPDC)"]
pub mod bpdc;
#[doc = "CTRLSTS (rw) register accessor: Control/status register (CRM_CTRLSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrlsts`]
module"]
pub type CTRLSTS = crate::Reg<ctrlsts::CTRLSTS_SPEC>;
#[doc = "Control/status register (CRM_CTRLSTS)"]
pub mod ctrlsts;
#[doc = "AHBRST (rw) register accessor: AHB reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ahbrst`]
module"]
pub type AHBRST = crate::Reg<ahbrst::AHBRST_SPEC>;
#[doc = "AHB reset register"]
pub mod ahbrst;
#[doc = "PLL (rw) register accessor: PLL configuration register (RCC_PLL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pll`]
module"]
pub type PLL = crate::Reg<pll::PLL_SPEC>;
#[doc = "PLL configuration register (RCC_PLL)"]
pub mod pll;
#[doc = "MISC1 (rw) register accessor: Miscellaneous register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`misc1`]
module"]
pub type MISC1 = crate::Reg<misc1::MISC1_SPEC>;
#[doc = "Miscellaneous register1"]
pub mod misc1;
#[doc = "OTG_EXTCTRL (rw) register accessor: OTGFS external ctrl register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_extctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_extctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`otg_extctrl`]
module"]
pub type OTG_EXTCTRL = crate::Reg<otg_extctrl::OTG_EXTCTRL_SPEC>;
#[doc = "OTGFS external ctrl register1"]
pub mod otg_extctrl;
#[doc = "MISC2 (rw) register accessor: Miscellaneous register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`misc2`]
module"]
pub type MISC2 = crate::Reg<misc2::MISC2_SPEC>;
#[doc = "Miscellaneous register2"]
pub mod misc2;
