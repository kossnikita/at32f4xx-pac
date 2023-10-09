#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    pub ctrl1: CTRL1,
    #[doc = "0x04 - Control register 2"]
    pub ctrl2: CTRL2,
    #[doc = "0x08 - Own address register 1"]
    pub oaddr1: OADDR1,
    #[doc = "0x0c - Own address register 2"]
    pub oaddr2: OADDR2,
    #[doc = "0x10 - Clock contorl register"]
    pub clkctrl: CLKCTRL,
    #[doc = "0x14 - Timeout register"]
    pub timeout: TIMEOUT,
    #[doc = "0x18 - Interrupt and Status register"]
    pub sts: STS,
    #[doc = "0x1c - Interrupt clear register"]
    pub clr: CLR,
    #[doc = "0x20 - PEC register"]
    pub pec: PEC,
    #[doc = "0x24 - Receive data register"]
    pub rxdt: RXDT,
    #[doc = "0x28 - Transmit data register"]
    pub txdt: TXDT,
}
#[doc = "CTRL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control register 2"]
pub mod ctrl2;
#[doc = "OADDR1 (rw) register accessor: Own address register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oaddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oaddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`oaddr1`]
module"]
pub type OADDR1 = crate::Reg<oaddr1::OADDR1_SPEC>;
#[doc = "Own address register 1"]
pub mod oaddr1;
#[doc = "OADDR2 (rw) register accessor: Own address register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oaddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oaddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`oaddr2`]
module"]
pub type OADDR2 = crate::Reg<oaddr2::OADDR2_SPEC>;
#[doc = "Own address register 2"]
pub mod oaddr2;
#[doc = "CLKCTRL (rw) register accessor: Clock contorl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkctrl`]
module"]
pub type CLKCTRL = crate::Reg<clkctrl::CLKCTRL_SPEC>;
#[doc = "Clock contorl register"]
pub mod clkctrl;
#[doc = "TIMEOUT (rw) register accessor: Timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timeout`]
module"]
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
#[doc = "Timeout register"]
pub mod timeout;
#[doc = "STS (rw) register accessor: Interrupt and Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "Interrupt and Status register"]
pub mod sts;
#[doc = "CLR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr`]
module"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod clr;
#[doc = "PEC (r) register accessor: PEC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pec::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pec`]
module"]
pub type PEC = crate::Reg<pec::PEC_SPEC>;
#[doc = "PEC register"]
pub mod pec;
#[doc = "RXDT (r) register accessor: Receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxdt`]
module"]
pub type RXDT = crate::Reg<rxdt::RXDT_SPEC>;
#[doc = "Receive data register"]
pub mod rxdt;
#[doc = "TXDT (rw) register accessor: Transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txdt`]
module"]
pub type TXDT = crate::Reg<txdt::TXDT_SPEC>;
#[doc = "Transmit data register"]
pub mod txdt;
