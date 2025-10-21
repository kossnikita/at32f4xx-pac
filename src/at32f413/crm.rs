#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    cfg: CFG,
    clkint: CLKINT,
    apb2rst: APB2RST,
    apb1rst: APB1RST,
    ahben: AHBEN,
    apb2en: APB2EN,
    apb1en: APB1EN,
    bpdc: BPDC,
    ctrlsts: CTRLSTS,
    _reserved10: [u8; 0x08],
    misc1: MISC1,
    _reserved11: [u8; 0x1c],
    misc2: MISC2,
    misc3: MISC3,
    _reserved13: [u8; 0x04],
    intmap: INTMAP,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Clock configuration register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &CFG {
        &self.cfg
    }
    #[doc = "0x08 - Clock interrupt register"]
    #[inline(always)]
    pub const fn clkint(&self) -> &CLKINT {
        &self.clkint
    }
    #[doc = "0x0c - APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rst(&self) -> &APB2RST {
        &self.apb2rst
    }
    #[doc = "0x10 - APB1 peripheral reset register"]
    #[inline(always)]
    pub const fn apb1rst(&self) -> &APB1RST {
        &self.apb1rst
    }
    #[doc = "0x14 - AHB Peripheral Clock enable register"]
    #[inline(always)]
    pub const fn ahben(&self) -> &AHBEN {
        &self.ahben
    }
    #[doc = "0x18 - APB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb2en(&self) -> &APB2EN {
        &self.apb2en
    }
    #[doc = "0x1c - APB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb1en(&self) -> &APB1EN {
        &self.apb1en
    }
    #[doc = "0x20 - Battery powered domain control register"]
    #[inline(always)]
    pub const fn bpdc(&self) -> &BPDC {
        &self.bpdc
    }
    #[doc = "0x24 - Control/status register"]
    #[inline(always)]
    pub const fn ctrlsts(&self) -> &CTRLSTS {
        &self.ctrlsts
    }
    #[doc = "0x30 - Miscellaneous register 1"]
    #[inline(always)]
    pub const fn misc1(&self) -> &MISC1 {
        &self.misc1
    }
    #[doc = "0x50 - Miscellaneous register 2"]
    #[inline(always)]
    pub const fn misc2(&self) -> &MISC2 {
        &self.misc2
    }
    #[doc = "0x54 - Miscellaneous register 3"]
    #[inline(always)]
    pub const fn misc3(&self) -> &MISC3 {
        &self.misc3
    }
    #[doc = "0x5c - Interrupt remap register"]
    #[inline(always)]
    pub const fn intmap(&self) -> &INTMAP {
        &self.intmap
    }
}
#[doc = "CTRL (rw) register accessor: Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Clock control register"]
pub mod ctrl;
#[doc = "CFG (rw) register accessor: Clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Clock configuration register"]
pub mod cfg;
#[doc = "CLKINT (rw) register accessor: Clock interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkint`] module"]
pub type CLKINT = crate::Reg<clkint::CLKINT_SPEC>;
#[doc = "Clock interrupt register"]
pub mod clkint;
#[doc = "APB2RST (rw) register accessor: APB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rst`] module"]
pub type APB2RST = crate::Reg<apb2rst::APB2RST_SPEC>;
#[doc = "APB2 peripheral reset register"]
pub mod apb2rst;
#[doc = "APB1RST (rw) register accessor: APB1 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rst`] module"]
pub type APB1RST = crate::Reg<apb1rst::APB1RST_SPEC>;
#[doc = "APB1 peripheral reset register"]
pub mod apb1rst;
#[doc = "AHBEN (rw) register accessor: AHB Peripheral Clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahben::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahben::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahben`] module"]
pub type AHBEN = crate::Reg<ahben::AHBEN_SPEC>;
#[doc = "AHB Peripheral Clock enable register"]
pub mod ahben;
#[doc = "APB2EN (rw) register accessor: APB2 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2en`] module"]
pub type APB2EN = crate::Reg<apb2en::APB2EN_SPEC>;
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2en;
#[doc = "APB1EN (rw) register accessor: APB1 peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1en`] module"]
pub type APB1EN = crate::Reg<apb1en::APB1EN_SPEC>;
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1en;
#[doc = "BPDC (rw) register accessor: Battery powered domain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bpdc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpdc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpdc`] module"]
pub type BPDC = crate::Reg<bpdc::BPDC_SPEC>;
#[doc = "Battery powered domain control register"]
pub mod bpdc;
#[doc = "CTRLSTS (rw) register accessor: Control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlsts`] module"]
pub type CTRLSTS = crate::Reg<ctrlsts::CTRLSTS_SPEC>;
#[doc = "Control/status register"]
pub mod ctrlsts;
#[doc = "MISC1 (rw) register accessor: Miscellaneous register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`misc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc1`] module"]
pub type MISC1 = crate::Reg<misc1::MISC1_SPEC>;
#[doc = "Miscellaneous register 1"]
pub mod misc1;
#[doc = "MISC2 (rw) register accessor: Miscellaneous register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`misc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc2`] module"]
pub type MISC2 = crate::Reg<misc2::MISC2_SPEC>;
#[doc = "Miscellaneous register 2"]
pub mod misc2;
#[doc = "MISC3 (rw) register accessor: Miscellaneous register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`misc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc3`] module"]
pub type MISC3 = crate::Reg<misc3::MISC3_SPEC>;
#[doc = "Miscellaneous register 3"]
pub mod misc3;
#[doc = "INTMAP (rw) register accessor: Interrupt remap register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmap`] module"]
pub type INTMAP = crate::Reg<intmap::INTMAP_SPEC>;
#[doc = "Interrupt remap register"]
pub mod intmap;
