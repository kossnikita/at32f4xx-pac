#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ctrl2: CTRL2,
    sts: STS,
    dt: DT,
    _reserved3: [u8; 0x0c],
    i2sctrl: I2SCTRL,
    i2sclk: I2SCLK,
    _reserved5: [u8; 0x0c],
    misc1: MISC1,
}
impl RegisterBlock {
    #[doc = "0x04 - control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x08 - status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &STS {
        &self.sts
    }
    #[doc = "0x0c - data register"]
    #[inline(always)]
    pub const fn dt(&self) -> &DT {
        &self.dt
    }
    #[doc = "0x1c - I2S control register"]
    #[inline(always)]
    pub const fn i2sctrl(&self) -> &I2SCTRL {
        &self.i2sctrl
    }
    #[doc = "0x20 - I2S clock register"]
    #[inline(always)]
    pub const fn i2sclk(&self) -> &I2SCLK {
        &self.i2sclk
    }
    #[doc = "0x30 - MISC1 register"]
    #[inline(always)]
    pub const fn misc1(&self) -> &MISC1 {
        &self.misc1
    }
}
#[doc = "CTRL2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "control register 2"]
pub mod ctrl2;
#[doc = "STS (rw) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "status register"]
pub mod sts;
#[doc = "DT (rw) register accessor: data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`]
module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "data register"]
pub mod dt;
#[doc = "I2SCTRL (rw) register accessor: I2S control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sctrl`]
module"]
pub type I2SCTRL = crate::Reg<i2sctrl::I2SCTRL_SPEC>;
#[doc = "I2S control register"]
pub mod i2sctrl;
#[doc = "I2SCLK (rw) register accessor: I2S clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sclk`]
module"]
pub type I2SCLK = crate::Reg<i2sclk::I2SCLK_SPEC>;
#[doc = "I2S clock register"]
pub mod i2sclk;
#[doc = "MISC1 (rw) register accessor: MISC1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc1`]
module"]
pub type MISC1 = crate::Reg<misc1::MISC1_SPEC>;
#[doc = "MISC1 register"]
pub mod misc1;
