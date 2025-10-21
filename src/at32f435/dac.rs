#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: CTRL,
    swtrg: SWTRG,
    d1dth12r: D1DTH12R,
    d1dth12l: D1DTH12L,
    d1dth8r: D1DTH8R,
    d2dth12r: D2DTH12R,
    d2dth12l: D2DTH12L,
    d2dth8r: D2DTH8R,
    ddth12r: DDTH12R,
    ddth12l: DDTH12L,
    ddth8r: DDTH8R,
    d1odt: D1ODT,
    d2odt: D2ODT,
    sts: STS,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register (DAC_CTRL)"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - DAC software trigger register(DAC_SWTRIGR)"]
    #[inline(always)]
    pub const fn swtrg(&self) -> &SWTRG {
        &self.swtrg
    }
    #[doc = "0x08 - DAC1 12-bit right-aligned data holding register(DAC_D1DTH12R)"]
    #[inline(always)]
    pub const fn d1dth12r(&self) -> &D1DTH12R {
        &self.d1dth12r
    }
    #[doc = "0x0c - DAC1 12-bit left aligned data holding register (DAC_D1DTH12L)"]
    #[inline(always)]
    pub const fn d1dth12l(&self) -> &D1DTH12L {
        &self.d1dth12l
    }
    #[doc = "0x10 - DAC1 8-bit right aligned data holding register (DAC_D1DTH8R)"]
    #[inline(always)]
    pub const fn d1dth8r(&self) -> &D1DTH8R {
        &self.d1dth8r
    }
    #[doc = "0x14 - DAC2 12-bit right aligned data holding register (DAC_D2DTH12R)"]
    #[inline(always)]
    pub const fn d2dth12r(&self) -> &D2DTH12R {
        &self.d2dth12r
    }
    #[doc = "0x18 - DAC2 12-bit left aligned data holding register (DAC_D2DTH12L)"]
    #[inline(always)]
    pub const fn d2dth12l(&self) -> &D2DTH12L {
        &self.d2dth12l
    }
    #[doc = "0x1c - DAC2 8-bit right-aligned data holding register (DAC_D2DTH8R)"]
    #[inline(always)]
    pub const fn d2dth8r(&self) -> &D2DTH8R {
        &self.d2dth8r
    }
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register (DAC_DDTH12R), Bits 31:28 Reserved, Bits 15:12 Reserved"]
    #[inline(always)]
    pub const fn ddth12r(&self) -> &DDTH12R {
        &self.ddth12r
    }
    #[doc = "0x24 - DUAL DAC 12-bit left aligned data holding register (DAC_DDTH12L), Bits 19:16 Reserved, Bits 3:0 Reserved"]
    #[inline(always)]
    pub const fn ddth12l(&self) -> &DDTH12L {
        &self.ddth12l
    }
    #[doc = "0x28 - DUAL DAC 8-bit right aligned data holding register (DAC_DDTH8R), Bits 31:16 Reserved"]
    #[inline(always)]
    pub const fn ddth8r(&self) -> &DDTH8R {
        &self.ddth8r
    }
    #[doc = "0x2c - DAC1 data output register (DAC_D1ODT)"]
    #[inline(always)]
    pub const fn d1odt(&self) -> &D1ODT {
        &self.d1odt
    }
    #[doc = "0x30 - DAC2 data output register (DAC_D2ODT)"]
    #[inline(always)]
    pub const fn d2odt(&self) -> &D2ODT {
        &self.d2odt
    }
    #[doc = "0x34 - DAC2 status register (DAC_STS)"]
    #[inline(always)]
    pub const fn sts(&self) -> &STS {
        &self.sts
    }
}
#[doc = "CTRL (rw) register accessor: Control register (DAC_CTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register (DAC_CTRL)"]
pub mod ctrl;
#[doc = "SWTRG (w) register accessor: DAC software trigger register(DAC_SWTRIGR)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrg`] module"]
pub type SWTRG = crate::Reg<swtrg::SWTRG_SPEC>;
#[doc = "DAC software trigger register(DAC_SWTRIGR)"]
pub mod swtrg;
#[doc = "D1DTH12R (rw) register accessor: DAC1 12-bit right-aligned data holding register(DAC_D1DTH12R)\n\nYou can [`read`](crate::Reg::read) this register and get [`d1dth12r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1dth12r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1dth12r`] module"]
pub type D1DTH12R = crate::Reg<d1dth12r::D1DTH12R_SPEC>;
#[doc = "DAC1 12-bit right-aligned data holding register(DAC_D1DTH12R)"]
pub mod d1dth12r;
#[doc = "D1DTH12L (rw) register accessor: DAC1 12-bit left aligned data holding register (DAC_D1DTH12L)\n\nYou can [`read`](crate::Reg::read) this register and get [`d1dth12l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1dth12l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1dth12l`] module"]
pub type D1DTH12L = crate::Reg<d1dth12l::D1DTH12L_SPEC>;
#[doc = "DAC1 12-bit left aligned data holding register (DAC_D1DTH12L)"]
pub mod d1dth12l;
#[doc = "D1DTH8R (rw) register accessor: DAC1 8-bit right aligned data holding register (DAC_D1DTH8R)\n\nYou can [`read`](crate::Reg::read) this register and get [`d1dth8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1dth8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1dth8r`] module"]
pub type D1DTH8R = crate::Reg<d1dth8r::D1DTH8R_SPEC>;
#[doc = "DAC1 8-bit right aligned data holding register (DAC_D1DTH8R)"]
pub mod d1dth8r;
#[doc = "D2DTH12R (rw) register accessor: DAC2 12-bit right aligned data holding register (DAC_D2DTH12R)\n\nYou can [`read`](crate::Reg::read) this register and get [`d2dth12r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2dth12r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2dth12r`] module"]
pub type D2DTH12R = crate::Reg<d2dth12r::D2DTH12R_SPEC>;
#[doc = "DAC2 12-bit right aligned data holding register (DAC_D2DTH12R)"]
pub mod d2dth12r;
#[doc = "D2DTH12L (rw) register accessor: DAC2 12-bit left aligned data holding register (DAC_D2DTH12L)\n\nYou can [`read`](crate::Reg::read) this register and get [`d2dth12l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2dth12l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2dth12l`] module"]
pub type D2DTH12L = crate::Reg<d2dth12l::D2DTH12L_SPEC>;
#[doc = "DAC2 12-bit left aligned data holding register (DAC_D2DTH12L)"]
pub mod d2dth12l;
#[doc = "D2DTH8R (rw) register accessor: DAC2 8-bit right-aligned data holding register (DAC_D2DTH8R)\n\nYou can [`read`](crate::Reg::read) this register and get [`d2dth8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2dth8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2dth8r`] module"]
pub type D2DTH8R = crate::Reg<d2dth8r::D2DTH8R_SPEC>;
#[doc = "DAC2 8-bit right-aligned data holding register (DAC_D2DTH8R)"]
pub mod d2dth8r;
#[doc = "DDTH12R (rw) register accessor: Dual DAC 12-bit right-aligned data holding register (DAC_DDTH12R), Bits 31:28 Reserved, Bits 15:12 Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ddth12r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddth12r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddth12r`] module"]
pub type DDTH12R = crate::Reg<ddth12r::DDTH12R_SPEC>;
#[doc = "Dual DAC 12-bit right-aligned data holding register (DAC_DDTH12R), Bits 31:28 Reserved, Bits 15:12 Reserved"]
pub mod ddth12r;
#[doc = "DDTH12L (rw) register accessor: DUAL DAC 12-bit left aligned data holding register (DAC_DDTH12L), Bits 19:16 Reserved, Bits 3:0 Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ddth12l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddth12l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddth12l`] module"]
pub type DDTH12L = crate::Reg<ddth12l::DDTH12L_SPEC>;
#[doc = "DUAL DAC 12-bit left aligned data holding register (DAC_DDTH12L), Bits 19:16 Reserved, Bits 3:0 Reserved"]
pub mod ddth12l;
#[doc = "DDTH8R (rw) register accessor: DUAL DAC 8-bit right aligned data holding register (DAC_DDTH8R), Bits 31:16 Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ddth8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddth8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddth8r`] module"]
pub type DDTH8R = crate::Reg<ddth8r::DDTH8R_SPEC>;
#[doc = "DUAL DAC 8-bit right aligned data holding register (DAC_DDTH8R), Bits 31:16 Reserved"]
pub mod ddth8r;
#[doc = "D1ODT (r) register accessor: DAC1 data output register (DAC_D1ODT)\n\nYou can [`read`](crate::Reg::read) this register and get [`d1odt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1odt`] module"]
pub type D1ODT = crate::Reg<d1odt::D1ODT_SPEC>;
#[doc = "DAC1 data output register (DAC_D1ODT)"]
pub mod d1odt;
#[doc = "D2ODT (r) register accessor: DAC2 data output register (DAC_D2ODT)\n\nYou can [`read`](crate::Reg::read) this register and get [`d2odt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2odt`] module"]
pub type D2ODT = crate::Reg<d2odt::D2ODT_SPEC>;
#[doc = "DAC2 data output register (DAC_D2ODT)"]
pub mod d2odt;
#[doc = "STS (rw) register accessor: DAC2 status register (DAC_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`] module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "DAC2 status register (DAC_STS)"]
pub mod sts;
