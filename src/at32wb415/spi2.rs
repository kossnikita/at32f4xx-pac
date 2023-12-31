#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub ctrl1: CTRL1,
    #[doc = "0x04 - control register 2"]
    pub ctrl2: CTRL2,
    #[doc = "0x08 - status register"]
    pub sts: STS,
    #[doc = "0x0c - data register"]
    pub dt: DT,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - CRC polynomial register"]
    pub cpoly: CPOLY,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - Receive CRC register"]
    pub rcrc: RCRC,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - Transmit CRC register"]
    pub tcrc: TCRC,
}
#[doc = "CTRL1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "control register 2"]
pub mod ctrl2;
#[doc = "STS (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "status register"]
pub mod sts;
#[doc = "DT (rw) register accessor: data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt`]
module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "data register"]
pub mod dt;
#[doc = "CPOLY (rw) register accessor: CRC polynomial register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpoly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpoly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpoly`]
module"]
pub type CPOLY = crate::Reg<cpoly::CPOLY_SPEC>;
#[doc = "CRC polynomial register"]
pub mod cpoly;
#[doc = "RCRC (r) register accessor: Receive CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcrc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rcrc`]
module"]
pub type RCRC = crate::Reg<rcrc::RCRC_SPEC>;
#[doc = "Receive CRC register"]
pub mod rcrc;
#[doc = "TCRC (r) register accessor: Transmit CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcrc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tcrc`]
module"]
pub type TCRC = crate::Reg<tcrc::TCRC_SPEC>;
#[doc = "Transmit CRC register"]
pub mod tcrc;
