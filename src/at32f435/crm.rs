#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    pllcfg: PLLCFG,
    cfg: CFG,
    clkint: CLKINT,
    ahbrst1: AHBRST1,
    ahbrst2: AHBRST2,
    ahbrst3: AHBRST3,
    _reserved7: [u8; 0x04],
    apb1rst: APB1RST,
    apb2rst: APB2RST,
    _reserved9: [u8; 0x08],
    ahben1: AHBEN1,
    ahben2: AHBEN2,
    ahben3: AHBEN3,
    _reserved12: [u8; 0x04],
    apb1en: APB1EN,
    apb2en: APB2EN,
    _reserved14: [u8; 0x08],
    ahblpen1: AHBLPEN1,
    ahblpen2: AHBLPEN2,
    ahblpen3: AHBLPEN3,
    _reserved17: [u8; 0x04],
    apb1lpen: APB1LPEN,
    apb2lpen: APB2LPEN,
    _reserved19: [u8; 0x08],
    bpdc: BPDC,
    ctrlsts: CTRLSTS,
    _reserved21: [u8; 0x28],
    misc1: MISC1,
    misc2: MISC2,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - PLL configuration register (CRM_PLLCFG)"]
    #[inline(always)]
    pub const fn pllcfg(&self) -> &PLLCFG {
        &self.pllcfg
    }
    #[doc = "0x08 - Clock configuration register(CRM_CFG)"]
    #[inline(always)]
    pub const fn cfg(&self) -> &CFG {
        &self.cfg
    }
    #[doc = "0x0c - Clock interrupt register (CRM_CLKINT)"]
    #[inline(always)]
    pub const fn clkint(&self) -> &CLKINT {
        &self.clkint
    }
    #[doc = "0x10 - AHB peripheral reset register1 (CRM_AHBRST1)"]
    #[inline(always)]
    pub const fn ahbrst1(&self) -> &AHBRST1 {
        &self.ahbrst1
    }
    #[doc = "0x14 - AHB peripheral reset register 2 (CRM_AHBRST2)"]
    #[inline(always)]
    pub const fn ahbrst2(&self) -> &AHBRST2 {
        &self.ahbrst2
    }
    #[doc = "0x18 - AHB peripheral reset register 3 (CRM_AHBRST3)"]
    #[inline(always)]
    pub const fn ahbrst3(&self) -> &AHBRST3 {
        &self.ahbrst3
    }
    #[doc = "0x20 - APB1 peripheral reset register (CRM_APB1RST)"]
    #[inline(always)]
    pub const fn apb1rst(&self) -> &APB1RST {
        &self.apb1rst
    }
    #[doc = "0x24 - APB2 peripheral reset register (CRM_APB2RST)"]
    #[inline(always)]
    pub const fn apb2rst(&self) -> &APB2RST {
        &self.apb2rst
    }
    #[doc = "0x30 - AHB Peripheral Clock enable register 1 (CRM_AHBEN1)"]
    #[inline(always)]
    pub const fn ahben1(&self) -> &AHBEN1 {
        &self.ahben1
    }
    #[doc = "0x34 - AHB peripheral clock enable register 2 (CRM_AHBEN2)"]
    #[inline(always)]
    pub const fn ahben2(&self) -> &AHBEN2 {
        &self.ahben2
    }
    #[doc = "0x38 - AHB peripheral clock enable register 3 (CRM_AHBEN3)"]
    #[inline(always)]
    pub const fn ahben3(&self) -> &AHBEN3 {
        &self.ahben3
    }
    #[doc = "0x40 - APB1 peripheral clock enable register (CRM_APB1EN)"]
    #[inline(always)]
    pub const fn apb1en(&self) -> &APB1EN {
        &self.apb1en
    }
    #[doc = "0x44 - APB2 peripheral clock enable register (CRM_APB2EN)"]
    #[inline(always)]
    pub const fn apb2en(&self) -> &APB2EN {
        &self.apb2en
    }
    #[doc = "0x50 - AHB Low-power Peripheral Clock enable register 1 (CRM_AHBLPEN1)"]
    #[inline(always)]
    pub const fn ahblpen1(&self) -> &AHBLPEN1 {
        &self.ahblpen1
    }
    #[doc = "0x54 - AHB peripheral Low-power clock enable register 2 (CRM_AHBLPEN2)"]
    #[inline(always)]
    pub const fn ahblpen2(&self) -> &AHBLPEN2 {
        &self.ahblpen2
    }
    #[doc = "0x58 - AHB peripheral Low-power clock enable register 3 (CRM_AHBLPEN3)"]
    #[inline(always)]
    pub const fn ahblpen3(&self) -> &AHBLPEN3 {
        &self.ahblpen3
    }
    #[doc = "0x60 - APB1 peripheral Low-power clock enable register (CRM_APB1LPEN)"]
    #[inline(always)]
    pub const fn apb1lpen(&self) -> &APB1LPEN {
        &self.apb1lpen
    }
    #[doc = "0x64 - APB2 peripheral Low-power clock enable register (CRM_APB2LPEN)"]
    #[inline(always)]
    pub const fn apb2lpen(&self) -> &APB2LPEN {
        &self.apb2lpen
    }
    #[doc = "0x70 - Battery powered domain control register (CRM_BPDC)"]
    #[inline(always)]
    pub const fn bpdc(&self) -> &BPDC {
        &self.bpdc
    }
    #[doc = "0x74 - Control/status register (CRM_CTRLSTS)"]
    #[inline(always)]
    pub const fn ctrlsts(&self) -> &CTRLSTS {
        &self.ctrlsts
    }
    #[doc = "0xa0 - Miscellaneous register1"]
    #[inline(always)]
    pub const fn misc1(&self) -> &MISC1 {
        &self.misc1
    }
    #[doc = "0xa4 - Miscellaneous register2"]
    #[inline(always)]
    pub const fn misc2(&self) -> &MISC2 {
        &self.misc2
    }
}
#[doc = "CTRL (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Clock control register"]
pub mod ctrl;
#[doc = "PLLCFG (rw) register accessor: PLL configuration register (CRM_PLLCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfg`]
module"]
pub type PLLCFG = crate::Reg<pllcfg::PLLCFG_SPEC>;
#[doc = "PLL configuration register (CRM_PLLCFG)"]
pub mod pllcfg;
#[doc = "CFG (rw) register accessor: Clock configuration register(CRM_CFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Clock configuration register(CRM_CFG)"]
pub mod cfg;
#[doc = "CLKINT (rw) register accessor: Clock interrupt register (CRM_CLKINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkint`]
module"]
pub type CLKINT = crate::Reg<clkint::CLKINT_SPEC>;
#[doc = "Clock interrupt register (CRM_CLKINT)"]
pub mod clkint;
#[doc = "AHBRST1 (rw) register accessor: AHB peripheral reset register1 (CRM_AHBRST1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrst1`]
module"]
pub type AHBRST1 = crate::Reg<ahbrst1::AHBRST1_SPEC>;
#[doc = "AHB peripheral reset register1 (CRM_AHBRST1)"]
pub mod ahbrst1;
#[doc = "AHBRST2 (rw) register accessor: AHB peripheral reset register 2 (CRM_AHBRST2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrst2`]
module"]
pub type AHBRST2 = crate::Reg<ahbrst2::AHBRST2_SPEC>;
#[doc = "AHB peripheral reset register 2 (CRM_AHBRST2)"]
pub mod ahbrst2;
#[doc = "AHBRST3 (rw) register accessor: AHB peripheral reset register 3 (CRM_AHBRST3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrst3`]
module"]
pub type AHBRST3 = crate::Reg<ahbrst3::AHBRST3_SPEC>;
#[doc = "AHB peripheral reset register 3 (CRM_AHBRST3)"]
pub mod ahbrst3;
#[doc = "APB1RST (rw) register accessor: APB1 peripheral reset register (CRM_APB1RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rst`]
module"]
pub type APB1RST = crate::Reg<apb1rst::APB1RST_SPEC>;
#[doc = "APB1 peripheral reset register (CRM_APB1RST)"]
pub mod apb1rst;
#[doc = "APB2RST (rw) register accessor: APB2 peripheral reset register (CRM_APB2RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rst`]
module"]
pub type APB2RST = crate::Reg<apb2rst::APB2RST_SPEC>;
#[doc = "APB2 peripheral reset register (CRM_APB2RST)"]
pub mod apb2rst;
#[doc = "AHBEN1 (rw) register accessor: AHB Peripheral Clock enable register 1 (CRM_AHBEN1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahben1`]
module"]
pub type AHBEN1 = crate::Reg<ahben1::AHBEN1_SPEC>;
#[doc = "AHB Peripheral Clock enable register 1 (CRM_AHBEN1)"]
pub mod ahben1;
#[doc = "AHBEN2 (rw) register accessor: AHB peripheral clock enable register 2 (CRM_AHBEN2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahben2`]
module"]
pub type AHBEN2 = crate::Reg<ahben2::AHBEN2_SPEC>;
#[doc = "AHB peripheral clock enable register 2 (CRM_AHBEN2)"]
pub mod ahben2;
#[doc = "AHBEN3 (rw) register accessor: AHB peripheral clock enable register 3 (CRM_AHBEN3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahben3`]
module"]
pub type AHBEN3 = crate::Reg<ahben3::AHBEN3_SPEC>;
#[doc = "AHB peripheral clock enable register 3 (CRM_AHBEN3)"]
pub mod ahben3;
#[doc = "APB1EN (rw) register accessor: APB1 peripheral clock enable register (CRM_APB1EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1en`]
module"]
pub type APB1EN = crate::Reg<apb1en::APB1EN_SPEC>;
#[doc = "APB1 peripheral clock enable register (CRM_APB1EN)"]
pub mod apb1en;
#[doc = "APB2EN (rw) register accessor: APB2 peripheral clock enable register (CRM_APB2EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2en`]
module"]
pub type APB2EN = crate::Reg<apb2en::APB2EN_SPEC>;
#[doc = "APB2 peripheral clock enable register (CRM_APB2EN)"]
pub mod apb2en;
#[doc = "AHBLPEN1 (rw) register accessor: AHB Low-power Peripheral Clock enable register 1 (CRM_AHBLPEN1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblpen1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblpen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblpen1`]
module"]
pub type AHBLPEN1 = crate::Reg<ahblpen1::AHBLPEN1_SPEC>;
#[doc = "AHB Low-power Peripheral Clock enable register 1 (CRM_AHBLPEN1)"]
pub mod ahblpen1;
#[doc = "AHBLPEN2 (rw) register accessor: AHB peripheral Low-power clock enable register 2 (CRM_AHBLPEN2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblpen2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblpen2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblpen2`]
module"]
pub type AHBLPEN2 = crate::Reg<ahblpen2::AHBLPEN2_SPEC>;
#[doc = "AHB peripheral Low-power clock enable register 2 (CRM_AHBLPEN2)"]
pub mod ahblpen2;
#[doc = "AHBLPEN3 (rw) register accessor: AHB peripheral Low-power clock enable register 3 (CRM_AHBLPEN3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblpen3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblpen3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahblpen3`]
module"]
pub type AHBLPEN3 = crate::Reg<ahblpen3::AHBLPEN3_SPEC>;
#[doc = "AHB peripheral Low-power clock enable register 3 (CRM_AHBLPEN3)"]
pub mod ahblpen3;
#[doc = "APB1LPEN (rw) register accessor: APB1 peripheral Low-power clock enable register (CRM_APB1LPEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lpen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lpen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lpen`]
module"]
pub type APB1LPEN = crate::Reg<apb1lpen::APB1LPEN_SPEC>;
#[doc = "APB1 peripheral Low-power clock enable register (CRM_APB1LPEN)"]
pub mod apb1lpen;
#[doc = "APB2LPEN (rw) register accessor: APB2 peripheral Low-power clock enable register (CRM_APB2LPEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2lpen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2lpen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2lpen`]
module"]
pub type APB2LPEN = crate::Reg<apb2lpen::APB2LPEN_SPEC>;
#[doc = "APB2 peripheral Low-power clock enable register (CRM_APB2LPEN)"]
pub mod apb2lpen;
#[doc = "BPDC (rw) register accessor: Battery powered domain control register (CRM_BPDC)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpdc`]
module"]
pub type BPDC = crate::Reg<bpdc::BPDC_SPEC>;
#[doc = "Battery powered domain control register (CRM_BPDC)"]
pub mod bpdc;
#[doc = "CTRLSTS (rw) register accessor: Control/status register (CRM_CTRLSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlsts`]
module"]
pub type CTRLSTS = crate::Reg<ctrlsts::CTRLSTS_SPEC>;
#[doc = "Control/status register (CRM_CTRLSTS)"]
pub mod ctrlsts;
#[doc = "MISC1 (rw) register accessor: Miscellaneous register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc1`]
module"]
pub type MISC1 = crate::Reg<misc1::MISC1_SPEC>;
#[doc = "Miscellaneous register1"]
pub mod misc1;
#[doc = "MISC2 (rw) register accessor: Miscellaneous register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc2`]
module"]
pub type MISC2 = crate::Reg<misc2::MISC2_SPEC>;
#[doc = "Miscellaneous register2"]
pub mod misc2;
