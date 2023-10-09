#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main control register"]
    pub mctrl: MCTRL,
    #[doc = "0x04 - Main status register"]
    pub msts: MSTS,
    #[doc = "0x08 - Transmit status register"]
    pub tsts: TSTS,
    #[doc = "0x0c..0x14 - Receive FIFO %s register"]
    pub rf: [RF; 2],
    #[doc = "0x14 - Interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x18 - Error status register"]
    pub ests: ESTS,
    #[doc = "0x1c - Bit timing register"]
    pub btmg: BTMG,
    _reserved7: [u8; 0x0160],
    #[doc = "0x180..0x1b0 - Mailbox %s"]
    pub mailbox: [MAILBOX; 3],
    #[doc = "0x1b0..0x1d0 - FIFO %s"]
    pub fifo: [FIFO; 2],
    _reserved9: [u8; 0x30],
    #[doc = "0x200 - Filter control register"]
    pub fctrl: FCTRL,
    #[doc = "0x204 - Filter mode config register"]
    pub fmcfg: FMCFG,
    _reserved11: [u8; 0x04],
    #[doc = "0x20c - Filter bit width config register"]
    pub fbwcfg: FBWCFG,
    _reserved12: [u8; 0x04],
    #[doc = "0x214 - Filter related FIFO register"]
    pub frf: FRF,
    _reserved13: [u8; 0x04],
    #[doc = "0x21c - Filter activate configuration register"]
    pub facfg: FACFG,
    _reserved14: [u8; 0x20],
    #[doc = "0x240..0x2b0 - Filter Bank %s"]
    pub filter_bank: [FILTER_BANK; 14],
}
#[doc = "MCTRL (rw) register accessor: Main control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mctrl`]
module"]
pub type MCTRL = crate::Reg<mctrl::MCTRL_SPEC>;
#[doc = "Main control register"]
pub mod mctrl;
#[doc = "MSTS (rw) register accessor: Main status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`msts`]
module"]
pub type MSTS = crate::Reg<msts::MSTS_SPEC>;
#[doc = "Main status register"]
pub mod msts;
#[doc = "TSTS (rw) register accessor: Transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tsts`]
module"]
pub type TSTS = crate::Reg<tsts::TSTS_SPEC>;
#[doc = "Transmit status register"]
pub mod tsts;
#[doc = "RF (rw) register accessor: Receive FIFO %s register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rf`]
module"]
pub type RF = crate::Reg<rf::RF_SPEC>;
#[doc = "Receive FIFO %s register"]
pub mod rf;
#[doc = "INTEN (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "ESTS (rw) register accessor: Error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ests::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ests::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ests`]
module"]
pub type ESTS = crate::Reg<ests::ESTS_SPEC>;
#[doc = "Error status register"]
pub mod ests;
#[doc = "BTMG (rw) register accessor: Bit timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btmg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btmg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`btmg`]
module"]
pub type BTMG = crate::Reg<btmg::BTMG_SPEC>;
#[doc = "Bit timing register"]
pub mod btmg;
#[doc = "Mailbox %s"]
pub use self::mailbox::MAILBOX;
#[doc = r"Cluster"]
#[doc = "Mailbox %s"]
pub mod mailbox;
#[doc = "FIFO %s"]
pub use self::fifo::FIFO;
#[doc = r"Cluster"]
#[doc = "FIFO %s"]
pub mod fifo;
#[doc = "FCTRL (rw) register accessor: Filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fctrl`]
module"]
pub type FCTRL = crate::Reg<fctrl::FCTRL_SPEC>;
#[doc = "Filter control register"]
pub mod fctrl;
#[doc = "FMCFG (rw) register accessor: Filter mode config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fmcfg`]
module"]
pub type FMCFG = crate::Reg<fmcfg::FMCFG_SPEC>;
#[doc = "Filter mode config register"]
pub mod fmcfg;
#[doc = "FBWCFG (rw) register accessor: Filter bit width config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbwcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbwcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fbwcfg`]
module"]
pub type FBWCFG = crate::Reg<fbwcfg::FBWCFG_SPEC>;
#[doc = "Filter bit width config register"]
pub mod fbwcfg;
#[doc = "FRF (rw) register accessor: Filter related FIFO register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`frf`]
module"]
pub type FRF = crate::Reg<frf::FRF_SPEC>;
#[doc = "Filter related FIFO register"]
pub mod frf;
#[doc = "FACFG (rw) register accessor: Filter activate configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`facfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`facfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`facfg`]
module"]
pub type FACFG = crate::Reg<facfg::FACFG_SPEC>;
#[doc = "Filter activate configuration register"]
pub mod facfg;
#[doc = "Filter Bank %s"]
pub use self::filter_bank::FILTER_BANK;
#[doc = r"Cluster"]
#[doc = "Filter Bank %s"]
pub mod filter_bank;
