#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    psr: PSR,
    unlock: UNLOCK,
    usd_unlock: USD_UNLOCK,
    sts: STS,
    ctrl: CTRL,
    addr: ADDR,
    _reserved6: [u8; 0x04],
    usd: USD,
    epps: EPPS,
    _reserved8: [u8; 0x50],
    slib_sts0: SLIB_STS0,
    slib_sts1: SLIB_STS1,
    slib_pwd_clr: SLIB_PWD_CLR,
    slib_misc_sts: SLIB_MISC_STS,
    crc_addr: CRC_ADDR,
    crc_ctrl: CRC_CTRL,
    crc_chkr: CRC_CHKR,
    _reserved15: [u8; 0xd0],
    slib_set_pwd: SLIB_SET_PWD,
    slib_set_range: SLIB_SET_RANGE,
    em_slib_set: EM_SLIB_SET,
    btm_mode_set: BTM_MODE_SET,
    slib_unlock: SLIB_UNLOCK,
}
impl RegisterBlock {
    #[doc = "0x00 - Performance selection register"]
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        &self.psr
    }
    #[doc = "0x04 - Unlock register"]
    #[inline(always)]
    pub const fn unlock(&self) -> &UNLOCK {
        &self.unlock
    }
    #[doc = "0x08 - USD unlock register"]
    #[inline(always)]
    pub const fn usd_unlock(&self) -> &USD_UNLOCK {
        &self.usd_unlock
    }
    #[doc = "0x0c - Status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &STS {
        &self.sts
    }
    #[doc = "0x10 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x14 - Address register"]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    #[doc = "0x1c - User system data register"]
    #[inline(always)]
    pub const fn usd(&self) -> &USD {
        &self.usd
    }
    #[doc = "0x20 - Erase/program protection status register"]
    #[inline(always)]
    pub const fn epps(&self) -> &EPPS {
        &self.epps
    }
    #[doc = "0x74 - sLib status 0 register"]
    #[inline(always)]
    pub const fn slib_sts0(&self) -> &SLIB_STS0 {
        &self.slib_sts0
    }
    #[doc = "0x78 - sLib status 1 register"]
    #[inline(always)]
    pub const fn slib_sts1(&self) -> &SLIB_STS1 {
        &self.slib_sts1
    }
    #[doc = "0x7c - SLIB password clear register"]
    #[inline(always)]
    pub const fn slib_pwd_clr(&self) -> &SLIB_PWD_CLR {
        &self.slib_pwd_clr
    }
    #[doc = "0x80 - sLib misc status register"]
    #[inline(always)]
    pub const fn slib_misc_sts(&self) -> &SLIB_MISC_STS {
        &self.slib_misc_sts
    }
    #[doc = "0x84 - Flash CRC data start address register"]
    #[inline(always)]
    pub const fn crc_addr(&self) -> &CRC_ADDR {
        &self.crc_addr
    }
    #[doc = "0x88 - Flash CRC controll register"]
    #[inline(always)]
    pub const fn crc_ctrl(&self) -> &CRC_CTRL {
        &self.crc_ctrl
    }
    #[doc = "0x8c - FLASH CRC check result register"]
    #[inline(always)]
    pub const fn crc_chkr(&self) -> &CRC_CHKR {
        &self.crc_chkr
    }
    #[doc = "0x160 - sLib password setting register"]
    #[inline(always)]
    pub const fn slib_set_pwd(&self) -> &SLIB_SET_PWD {
        &self.slib_set_pwd
    }
    #[doc = "0x164 - Configure sLib range register"]
    #[inline(always)]
    pub const fn slib_set_range(&self) -> &SLIB_SET_RANGE {
        &self.slib_set_range
    }
    #[doc = "0x168 - Extension momery slib set register"]
    #[inline(always)]
    pub const fn em_slib_set(&self) -> &EM_SLIB_SET {
        &self.em_slib_set
    }
    #[doc = "0x16c - Boot memory mode setting register"]
    #[inline(always)]
    pub const fn btm_mode_set(&self) -> &BTM_MODE_SET {
        &self.btm_mode_set
    }
    #[doc = "0x170 - sLib unlock register"]
    #[inline(always)]
    pub const fn slib_unlock(&self) -> &SLIB_UNLOCK {
        &self.slib_unlock
    }
}
#[doc = "PSR (rw) register accessor: Performance selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Performance selection register"]
pub mod psr;
#[doc = "UNLOCK (w) register accessor: Unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unlock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unlock`]
module"]
pub type UNLOCK = crate::Reg<unlock::UNLOCK_SPEC>;
#[doc = "Unlock register"]
pub mod unlock;
#[doc = "USD_UNLOCK (w) register accessor: USD unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usd_unlock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usd_unlock`]
module"]
pub type USD_UNLOCK = crate::Reg<usd_unlock::USD_UNLOCK_SPEC>;
#[doc = "USD unlock register"]
pub mod usd_unlock;
#[doc = "STS (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "Status register"]
pub mod sts;
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "ADDR (w) register accessor: Address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address register"]
pub mod addr;
#[doc = "USD (r) register accessor: User system data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usd`]
module"]
pub type USD = crate::Reg<usd::USD_SPEC>;
#[doc = "User system data register"]
pub mod usd;
#[doc = "EPPS (r) register accessor: Erase/program protection status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epps::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epps`]
module"]
pub type EPPS = crate::Reg<epps::EPPS_SPEC>;
#[doc = "Erase/program protection status register"]
pub mod epps;
#[doc = "SLIB_STS0 (rw) register accessor: sLib status 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_sts0`]
module"]
pub type SLIB_STS0 = crate::Reg<slib_sts0::SLIB_STS0_SPEC>;
#[doc = "sLib status 0 register"]
pub mod slib_sts0;
#[doc = "SLIB_STS1 (rw) register accessor: sLib status 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_sts1`]
module"]
pub type SLIB_STS1 = crate::Reg<slib_sts1::SLIB_STS1_SPEC>;
#[doc = "sLib status 1 register"]
pub mod slib_sts1;
#[doc = "SLIB_PWD_CLR (w) register accessor: SLIB password clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_pwd_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_pwd_clr`]
module"]
pub type SLIB_PWD_CLR = crate::Reg<slib_pwd_clr::SLIB_PWD_CLR_SPEC>;
#[doc = "SLIB password clear register"]
pub mod slib_pwd_clr;
#[doc = "SLIB_MISC_STS (rw) register accessor: sLib misc status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_misc_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_misc_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_misc_sts`]
module"]
pub type SLIB_MISC_STS = crate::Reg<slib_misc_sts::SLIB_MISC_STS_SPEC>;
#[doc = "sLib misc status register"]
pub mod slib_misc_sts;
#[doc = "CRC_ADDR (w) register accessor: Flash CRC data start address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_addr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_addr`]
module"]
pub type CRC_ADDR = crate::Reg<crc_addr::CRC_ADDR_SPEC>;
#[doc = "Flash CRC data start address register"]
pub mod crc_addr;
#[doc = "CRC_CTRL (rw) register accessor: Flash CRC controll register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_ctrl`]
module"]
pub type CRC_CTRL = crate::Reg<crc_ctrl::CRC_CTRL_SPEC>;
#[doc = "Flash CRC controll register"]
pub mod crc_ctrl;
#[doc = "CRC_CHKR (r) register accessor: FLASH CRC check result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_chkr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_chkr`]
module"]
pub type CRC_CHKR = crate::Reg<crc_chkr::CRC_CHKR_SPEC>;
#[doc = "FLASH CRC check result register"]
pub mod crc_chkr;
#[doc = "SLIB_SET_PWD (w) register accessor: sLib password setting register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_pwd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_set_pwd`]
module"]
pub type SLIB_SET_PWD = crate::Reg<slib_set_pwd::SLIB_SET_PWD_SPEC>;
#[doc = "sLib password setting register"]
pub mod slib_set_pwd;
#[doc = "SLIB_SET_RANGE (w) register accessor: Configure sLib range register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_range::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_set_range`]
module"]
pub type SLIB_SET_RANGE = crate::Reg<slib_set_range::SLIB_SET_RANGE_SPEC>;
#[doc = "Configure sLib range register"]
pub mod slib_set_range;
#[doc = "EM_SLIB_SET (w) register accessor: Extension momery slib set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em_slib_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em_slib_set`]
module"]
pub type EM_SLIB_SET = crate::Reg<em_slib_set::EM_SLIB_SET_SPEC>;
#[doc = "Extension momery slib set register"]
pub mod em_slib_set;
#[doc = "BTM_MODE_SET (w) register accessor: Boot memory mode setting register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btm_mode_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btm_mode_set`]
module"]
pub type BTM_MODE_SET = crate::Reg<btm_mode_set::BTM_MODE_SET_SPEC>;
#[doc = "Boot memory mode setting register"]
pub mod btm_mode_set;
#[doc = "SLIB_UNLOCK (w) register accessor: sLib unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_unlock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_unlock`]
module"]
pub type SLIB_UNLOCK = crate::Reg<slib_unlock::SLIB_UNLOCK_SPEC>;
#[doc = "sLib unlock register"]
pub mod slib_unlock;
