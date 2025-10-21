#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl1: CTRL1,
    ctrl2: CTRL2,
    sts: STS,
    dt: DT,
    _reserved4: [u8; 0x02],
    cpoly: CPOLY,
    _reserved5: [u8; 0x02],
    rcrc: RCRC,
    _reserved6: [u8; 0x02],
    tcrc: TCRC,
    _reserved7: [u8; 0x02],
    i2sctrl: I2SCTRL,
    i2sclk: I2SCLK,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
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
    #[doc = "0x10 - CRC polynomial register"]
    #[inline(always)]
    pub const fn cpoly(&self) -> &CPOLY {
        &self.cpoly
    }
    #[doc = "0x14 - Receive CRC register"]
    #[inline(always)]
    pub const fn rcrc(&self) -> &RCRC {
        &self.rcrc
    }
    #[doc = "0x18 - Transmit CRC register"]
    #[inline(always)]
    pub const fn tcrc(&self) -> &TCRC {
        &self.tcrc
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
}
#[doc = "CTRL1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "control register 2"]
pub mod ctrl2;
#[doc = "STS (rw) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`] module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "status register"]
pub mod sts;
#[doc = "DT (rw) register accessor: data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`] module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "data register"]
pub mod dt;
#[doc = "CPOLY (rw) register accessor: CRC polynomial register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpoly`] module"]
pub type CPOLY = crate::Reg<cpoly::CPOLY_SPEC>;
#[doc = "CRC polynomial register"]
pub mod cpoly;
#[doc = "RCRC (r) register accessor: Receive CRC register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcrc`] module"]
pub type RCRC = crate::Reg<rcrc::RCRC_SPEC>;
#[doc = "Receive CRC register"]
pub mod rcrc;
#[doc = "TCRC (r) register accessor: Transmit CRC register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcrc`] module"]
pub type TCRC = crate::Reg<tcrc::TCRC_SPEC>;
#[doc = "Transmit CRC register"]
pub mod tcrc;
#[doc = "I2SCTRL (rw) register accessor: I2S control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sctrl`] module"]
pub type I2SCTRL = crate::Reg<i2sctrl::I2SCTRL_SPEC>;
#[doc = "I2S control register"]
pub mod i2sctrl;
#[doc = "I2SCLK (rw) register accessor: I2S clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sclk`] module"]
pub type I2SCLK = crate::Reg<i2sclk::I2SCLK_SPEC>;
#[doc = "I2S clock register"]
pub mod i2sclk;
