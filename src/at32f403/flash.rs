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
    _reserved8: [u8; 0x20],
    unlock2: UNLOCK2,
    _reserved9: [u8; 0x04],
    sts2: STS2,
    ctrl2: CTRL2,
    addr2: ADDR2,
    _reserved12: [u8; 0x2c],
    unlock3: UNLOCK3,
    select: SELECT,
    sts3: STS3,
    ctrl3: CTRL3,
    addr3: ADDR3,
    da: DA,
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
    #[doc = "0x44 - Unlock 2 register"]
    #[inline(always)]
    pub const fn unlock2(&self) -> &UNLOCK2 {
        &self.unlock2
    }
    #[doc = "0x4c - Status 2 register"]
    #[inline(always)]
    pub const fn sts2(&self) -> &STS2 {
        &self.sts2
    }
    #[doc = "0x50 - Control 2 register"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x54 - Address 2 register"]
    #[inline(always)]
    pub const fn addr2(&self) -> &ADDR2 {
        &self.addr2
    }
    #[doc = "0x84 - Unlock 3 register"]
    #[inline(always)]
    pub const fn unlock3(&self) -> &UNLOCK3 {
        &self.unlock3
    }
    #[doc = "0x88 - Select register"]
    #[inline(always)]
    pub const fn select(&self) -> &SELECT {
        &self.select
    }
    #[doc = "0x8c - Status 3 register"]
    #[inline(always)]
    pub const fn sts3(&self) -> &STS3 {
        &self.sts3
    }
    #[doc = "0x90 - Control 3 register"]
    #[inline(always)]
    pub const fn ctrl3(&self) -> &CTRL3 {
        &self.ctrl3
    }
    #[doc = "0x94 - Address 3 register"]
    #[inline(always)]
    pub const fn addr3(&self) -> &ADDR3 {
        &self.addr3
    }
    #[doc = "0x98 - Spim decryption address"]
    #[inline(always)]
    pub const fn da(&self) -> &DA {
        &self.da
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
#[doc = "UNLOCK2 (w) register accessor: Unlock 2 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unlock2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unlock2`]
module"]
pub type UNLOCK2 = crate::Reg<unlock2::UNLOCK2_SPEC>;
#[doc = "Unlock 2 register"]
pub mod unlock2;
#[doc = "STS2 (rw) register accessor: Status 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts2`]
module"]
pub type STS2 = crate::Reg<sts2::STS2_SPEC>;
#[doc = "Status 2 register"]
pub mod sts2;
#[doc = "CTRL2 (rw) register accessor: Control 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control 2 register"]
pub mod ctrl2;
#[doc = "ADDR2 (w) register accessor: Address 2 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr2`]
module"]
pub type ADDR2 = crate::Reg<addr2::ADDR2_SPEC>;
#[doc = "Address 2 register"]
pub mod addr2;
#[doc = "UNLOCK3 (w) register accessor: Unlock 3 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unlock3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unlock3`]
module"]
pub type UNLOCK3 = crate::Reg<unlock3::UNLOCK3_SPEC>;
#[doc = "Unlock 3 register"]
pub mod unlock3;
#[doc = "SELECT (w) register accessor: Select register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`select::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@select`]
module"]
pub type SELECT = crate::Reg<select::SELECT_SPEC>;
#[doc = "Select register"]
pub mod select;
#[doc = "STS3 (rw) register accessor: Status 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts3`]
module"]
pub type STS3 = crate::Reg<sts3::STS3_SPEC>;
#[doc = "Status 3 register"]
pub mod sts3;
#[doc = "CTRL3 (rw) register accessor: Control 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl3`]
module"]
pub type CTRL3 = crate::Reg<ctrl3::CTRL3_SPEC>;
#[doc = "Control 3 register"]
pub mod ctrl3;
#[doc = "ADDR3 (w) register accessor: Address 3 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr3`]
module"]
pub type ADDR3 = crate::Reg<addr3::ADDR3_SPEC>;
#[doc = "Address 3 register"]
pub mod addr3;
#[doc = "DA (w) register accessor: Spim decryption address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`da::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@da`]
module"]
pub type DA = crate::Reg<da::DA_SPEC>;
#[doc = "Spim decryption address"]
pub mod da;
