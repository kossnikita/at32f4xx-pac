#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl1: CTRL1,
    ctrl2: CTRL2,
    oaddr1: OADDR1,
    oaddr2: OADDR2,
    dt: DT,
    sts1: STS1,
    sts2: STS2,
    clkctrl: CLKCTRL,
    tmrise: TMRISE,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x04 - Control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x08 - Own address register 1"]
    #[inline(always)]
    pub const fn oaddr1(&self) -> &OADDR1 {
        &self.oaddr1
    }
    #[doc = "0x0c - Own address register 2"]
    #[inline(always)]
    pub const fn oaddr2(&self) -> &OADDR2 {
        &self.oaddr2
    }
    #[doc = "0x10 - Data register"]
    #[inline(always)]
    pub const fn dt(&self) -> &DT {
        &self.dt
    }
    #[doc = "0x14 - Status register 1"]
    #[inline(always)]
    pub const fn sts1(&self) -> &STS1 {
        &self.sts1
    }
    #[doc = "0x18 - Status register 2"]
    #[inline(always)]
    pub const fn sts2(&self) -> &STS2 {
        &self.sts2
    }
    #[doc = "0x1c - Clock control register"]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &CLKCTRL {
        &self.clkctrl
    }
    #[doc = "0x20 - TRISE register"]
    #[inline(always)]
    pub const fn tmrise(&self) -> &TMRISE {
        &self.tmrise
    }
}
#[doc = "CTRL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control register 2"]
pub mod ctrl2;
#[doc = "OADDR1 (rw) register accessor: Own address register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`oaddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oaddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oaddr1`] module"]
pub type OADDR1 = crate::Reg<oaddr1::OADDR1_SPEC>;
#[doc = "Own address register 1"]
pub mod oaddr1;
#[doc = "OADDR2 (rw) register accessor: Own address register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`oaddr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oaddr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oaddr2`] module"]
pub type OADDR2 = crate::Reg<oaddr2::OADDR2_SPEC>;
#[doc = "Own address register 2"]
pub mod oaddr2;
#[doc = "DT (rw) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`] module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "Data register"]
pub mod dt;
#[doc = "STS1 (rw) register accessor: Status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sts1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts1`] module"]
pub type STS1 = crate::Reg<sts1::STS1_SPEC>;
#[doc = "Status register 1"]
pub mod sts1;
#[doc = "STS2 (r) register accessor: Status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts2`] module"]
pub type STS2 = crate::Reg<sts2::STS2_SPEC>;
#[doc = "Status register 2"]
pub mod sts2;
#[doc = "CLKCTRL (rw) register accessor: Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`] module"]
pub type CLKCTRL = crate::Reg<clkctrl::CLKCTRL_SPEC>;
#[doc = "Clock control register"]
pub mod clkctrl;
#[doc = "TMRISE (rw) register accessor: TRISE register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmrise::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmrise::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmrise`] module"]
pub type TMRISE = crate::Reg<tmrise::TMRISE_SPEC>;
#[doc = "TRISE register"]
pub mod tmrise;
