#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
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
