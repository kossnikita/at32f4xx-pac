#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Performance selection register"]
    pub psr: PSR,
    #[doc = "0x04 - Unlock register"]
    pub unlock: UNLOCK,
    #[doc = "0x08 - USD unlock register"]
    pub usd_unlock: USD_UNLOCK,
    #[doc = "0x0c - Status register"]
    pub sts: STS,
    #[doc = "0x10 - Control register"]
    pub ctrl: CTRL,
    #[doc = "0x14 - Address register"]
    pub addr: ADDR,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - User system data register"]
    pub usd: USD,
    #[doc = "0x20 - Erase/program protection status register"]
    pub epps: EPPS,
    _reserved8: [u8; 0x20],
    #[doc = "0x44 - Unlock 2 register"]
    pub unlock2: UNLOCK2,
    _reserved9: [u8; 0x04],
    #[doc = "0x4c - Status 2 register"]
    pub sts2: STS2,
    #[doc = "0x50 - Control 2 register"]
    pub ctrl2: CTRL2,
    #[doc = "0x54 - Address 2 register"]
    pub addr2: ADDR2,
    _reserved12: [u8; 0x2c],
    #[doc = "0x84 - Unlock 3 register"]
    pub unlock3: UNLOCK3,
    #[doc = "0x88 - Select register"]
    pub select: SELECT,
    #[doc = "0x8c - Status 3 register"]
    pub sts3: STS3,
    #[doc = "0x90 - Control 3 register"]
    pub ctrl3: CTRL3,
    #[doc = "0x94 - Address 3 register"]
    pub addr3: ADDR3,
    #[doc = "0x98 - Spim decryption address"]
    pub da: DA,
    _reserved18: [u8; 0x30],
    #[doc = "0xcc - sLib status 0 register"]
    pub slib_sts0: SLIB_STS0,
    #[doc = "0xd0 - sLib status 1 register"]
    pub slib_sts1: SLIB_STS1,
    #[doc = "0xd4 - SLIB password clear register"]
    pub slib_pwd_clr: SLIB_PWD_CLR,
    #[doc = "0xd8 - sLib misc status register"]
    pub slib_misc_sts: SLIB_MISC_STS,
    #[doc = "0xdc - sLib password setting register"]
    pub slib_set_pwd: SLIB_SET_PWD,
    #[doc = "0xe0 - Configure sLib range register"]
    pub slib_set_range: SLIB_SET_RANGE,
    _reserved24: [u8; 0x0c],
    #[doc = "0xf0 - sLib unlock register"]
    pub slib_unlock: SLIB_UNLOCK,
    #[doc = "0xf4 - CRC controler register"]
    pub crc_ctrl: CRC_CTRL,
    #[doc = "0xf8 - CRC check result register"]
    pub crc_chkr: CRC_CHKR,
}
#[doc = "PSR (rw) register accessor: Performance selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`psr`]
module"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Performance selection register"]
pub mod psr;
#[doc = "UNLOCK (w) register accessor: Unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unlock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`unlock`]
module"]
pub type UNLOCK = crate::Reg<unlock::UNLOCK_SPEC>;
#[doc = "Unlock register"]
pub mod unlock;
#[doc = "USD_UNLOCK (w) register accessor: USD unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usd_unlock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`usd_unlock`]
module"]
pub type USD_UNLOCK = crate::Reg<usd_unlock::USD_UNLOCK_SPEC>;
#[doc = "USD unlock register"]
pub mod usd_unlock;
#[doc = "STS (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "Status register"]
pub mod sts;
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "ADDR (w) register accessor: Address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address register"]
pub mod addr;
#[doc = "USD (r) register accessor: User system data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`usd`]
module"]
pub type USD = crate::Reg<usd::USD_SPEC>;
#[doc = "User system data register"]
pub mod usd;
#[doc = "EPPS (r) register accessor: Erase/program protection status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epps::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`epps`]
module"]
pub type EPPS = crate::Reg<epps::EPPS_SPEC>;
#[doc = "Erase/program protection status register"]
pub mod epps;
#[doc = "UNLOCK2 (w) register accessor: Unlock 2 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unlock2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`unlock2`]
module"]
pub type UNLOCK2 = crate::Reg<unlock2::UNLOCK2_SPEC>;
#[doc = "Unlock 2 register"]
pub mod unlock2;
#[doc = "STS2 (rw) register accessor: Status 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts2`]
module"]
pub type STS2 = crate::Reg<sts2::STS2_SPEC>;
#[doc = "Status 2 register"]
pub mod sts2;
#[doc = "CTRL2 (rw) register accessor: Control 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control 2 register"]
pub mod ctrl2;
#[doc = "ADDR2 (w) register accessor: Address 2 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addr2`]
module"]
pub type ADDR2 = crate::Reg<addr2::ADDR2_SPEC>;
#[doc = "Address 2 register"]
pub mod addr2;
#[doc = "UNLOCK3 (w) register accessor: Unlock 3 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unlock3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`unlock3`]
module"]
pub type UNLOCK3 = crate::Reg<unlock3::UNLOCK3_SPEC>;
#[doc = "Unlock 3 register"]
pub mod unlock3;
#[doc = "SELECT (w) register accessor: Select register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`select::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`select`]
module"]
pub type SELECT = crate::Reg<select::SELECT_SPEC>;
#[doc = "Select register"]
pub mod select;
#[doc = "STS3 (rw) register accessor: Status 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts3`]
module"]
pub type STS3 = crate::Reg<sts3::STS3_SPEC>;
#[doc = "Status 3 register"]
pub mod sts3;
#[doc = "CTRL3 (rw) register accessor: Control 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl3`]
module"]
pub type CTRL3 = crate::Reg<ctrl3::CTRL3_SPEC>;
#[doc = "Control 3 register"]
pub mod ctrl3;
#[doc = "ADDR3 (w) register accessor: Address 3 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addr3`]
module"]
pub type ADDR3 = crate::Reg<addr3::ADDR3_SPEC>;
#[doc = "Address 3 register"]
pub mod addr3;
#[doc = "DA (w) register accessor: Spim decryption address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`da::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`da`]
module"]
pub type DA = crate::Reg<da::DA_SPEC>;
#[doc = "Spim decryption address"]
pub mod da;
#[doc = "SLIB_STS0 (rw) register accessor: sLib status 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slib_sts0`]
module"]
pub type SLIB_STS0 = crate::Reg<slib_sts0::SLIB_STS0_SPEC>;
#[doc = "sLib status 0 register"]
pub mod slib_sts0;
#[doc = "SLIB_STS1 (rw) register accessor: sLib status 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slib_sts1`]
module"]
pub type SLIB_STS1 = crate::Reg<slib_sts1::SLIB_STS1_SPEC>;
#[doc = "sLib status 1 register"]
pub mod slib_sts1;
#[doc = "SLIB_PWD_CLR (w) register accessor: SLIB password clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_pwd_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slib_pwd_clr`]
module"]
pub type SLIB_PWD_CLR = crate::Reg<slib_pwd_clr::SLIB_PWD_CLR_SPEC>;
#[doc = "SLIB password clear register"]
pub mod slib_pwd_clr;
#[doc = "SLIB_MISC_STS (rw) register accessor: sLib misc status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_misc_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_misc_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slib_misc_sts`]
module"]
pub type SLIB_MISC_STS = crate::Reg<slib_misc_sts::SLIB_MISC_STS_SPEC>;
#[doc = "sLib misc status register"]
pub mod slib_misc_sts;
#[doc = "SLIB_SET_PWD (w) register accessor: sLib password setting register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_pwd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slib_set_pwd`]
module"]
pub type SLIB_SET_PWD = crate::Reg<slib_set_pwd::SLIB_SET_PWD_SPEC>;
#[doc = "sLib password setting register"]
pub mod slib_set_pwd;
#[doc = "SLIB_SET_RANGE (w) register accessor: Configure sLib range register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_range::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slib_set_range`]
module"]
pub type SLIB_SET_RANGE = crate::Reg<slib_set_range::SLIB_SET_RANGE_SPEC>;
#[doc = "Configure sLib range register"]
pub mod slib_set_range;
#[doc = "SLIB_UNLOCK (w) register accessor: sLib unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_unlock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slib_unlock`]
module"]
pub type SLIB_UNLOCK = crate::Reg<slib_unlock::SLIB_UNLOCK_SPEC>;
#[doc = "sLib unlock register"]
pub mod slib_unlock;
#[doc = "CRC_CTRL (w) register accessor: CRC controler register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`crc_ctrl`]
module"]
pub type CRC_CTRL = crate::Reg<crc_ctrl::CRC_CTRL_SPEC>;
#[doc = "CRC controler register"]
pub mod crc_ctrl;
#[doc = "CRC_CHKR (r) register accessor: CRC check result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_chkr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`crc_chkr`]
module"]
pub type CRC_CHKR = crate::Reg<crc_chkr::CRC_CHKR_SPEC>;
#[doc = "CRC check result register"]
pub mod crc_chkr;
