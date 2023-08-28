#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main control register"]
    pub mctrl: MCTRL,
    #[doc = "0x04 - Main status register"]
    pub msts: MSTS,
    #[doc = "0x08 - Transmit status register"]
    pub tsts: TSTS,
    #[doc = "0x0c - Receive FIFO 0 register"]
    pub rf0: RF0,
    #[doc = "0x10 - Receive FIFO 1 register"]
    pub rf1: RF1,
    #[doc = "0x14 - Interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x18 - Error status register"]
    pub ests: ESTS,
    #[doc = "0x1c - Bit timing register"]
    pub btmg: BTMG,
    _reserved8: [u8; 0x0160],
    #[doc = "0x180 - Transmit mailbox 0 identifier register"]
    pub tmi0: TMI0,
    #[doc = "0x184 - Transmit mailbox 0 data length and time stamp register"]
    pub tmc0: TMC0,
    #[doc = "0x188 - Transmit mailbox 0 low byte data register"]
    pub tmdtl0: TMDTL0,
    #[doc = "0x18c - Transmit mailbox 0 high byte data register"]
    pub tmdth0: TMDTH0,
    #[doc = "0x190 - Transmit mailbox 1 identifier register"]
    pub tmi1: TMI1,
    #[doc = "0x194 - Transmit mailbox 1 data length and time stamp register"]
    pub tmc1: TMC1,
    #[doc = "0x198 - Transmit mailbox 1 low byte data register"]
    pub tmdtl1: TMDTL1,
    #[doc = "0x19c - Transmit mailbox 1 high byte data register"]
    pub tmdth1: TMDTH1,
    #[doc = "0x1a0 - Transmit mailbox 2 identifier register"]
    pub tmi2: TMI2,
    #[doc = "0x1a4 - Transmit mailbox 2 data length and time stamp register"]
    pub tmc2: TMC2,
    #[doc = "0x1a8 - Transmit mailbox 2 low byte data register"]
    pub tmdtl2: TMDTL2,
    #[doc = "0x1ac - Transmit mailbox 2 high byte data register"]
    pub tmdth2: TMDTH2,
    #[doc = "0x1b0 - Receive FIFO 0 register"]
    pub rfi0: RFI0,
    #[doc = "0x1b4 - Receive FIFO 0 data length and time stamp register"]
    pub rfc0: RFC0,
    #[doc = "0x1b8 - Receive FIFO 0 low byte data register"]
    pub rfdtl0: RFDTL0,
    #[doc = "0x1bc - Receive FIFO 0 high byte data register"]
    pub rfdth0: RFDTH0,
    #[doc = "0x1c0 - Receive FIFO 1 register"]
    pub rfi1: RFI1,
    #[doc = "0x1c4 - Receive FIFO 1 data length and time stamp register"]
    pub rfc1: RFC1,
    #[doc = "0x1c8 - Receive FIFO 1 low byte data register"]
    pub rfdtl1: RFDTL1,
    #[doc = "0x1cc - Receive FIFO 1 high byte data register"]
    pub rfdth1: RFDTH1,
    _reserved28: [u8; 0x30],
    #[doc = "0x200 - Filter control register"]
    pub fctrl: FCTRL,
    #[doc = "0x204 - Filter mode config register"]
    pub fmcfg: FMCFG,
    _reserved30: [u8; 0x04],
    #[doc = "0x20c - Filter bit width config register"]
    pub fbwcfg: FBWCFG,
    _reserved31: [u8; 0x04],
    #[doc = "0x214 - Filter related FIFO register"]
    pub frf: FRF,
    _reserved32: [u8; 0x04],
    #[doc = "0x21c - Filter activate configuration register"]
    pub facfg: FACFG,
    _reserved33: [u8; 0x20],
    #[doc = "0x240 - Filter bank 0 filtrate bit register 1"]
    pub f0fb1: F0FB1,
    #[doc = "0x244 - Filter bank 0 filtrate bit register 2"]
    pub f0fb2: F0FB2,
    #[doc = "0x248 - Filter bank 1 filtrate bit register 1"]
    pub f1fb1: F1FB1,
    #[doc = "0x24c - Filter bank 1 filtrate bit register 2"]
    pub f1fb2: F1FB2,
    #[doc = "0x250 - Filter bank 2 filtrate bit register 1"]
    pub f2fb1: F2FB1,
    #[doc = "0x254 - Filter bank 2 filtrate bit register 2"]
    pub f2fb2: F2FB2,
    #[doc = "0x258 - Filter bank 3 filtrate bit register 1"]
    pub f3fb1: F3FB1,
    #[doc = "0x25c - Filter bank 3 filtrate bit register 2"]
    pub f3fb2: F3FB2,
    #[doc = "0x260 - Filter bank 4 filtrate bit register 1"]
    pub f4fb1: F4FB1,
    #[doc = "0x264 - Filter bank 4 filtrate bit register 2"]
    pub f4fb2: F4FB2,
    #[doc = "0x268 - Filter bank 5 filtrate bit register 1"]
    pub f5fb1: F5FB1,
    #[doc = "0x26c - Filter bank 5 filtrate bit register 2"]
    pub f5fb2: F5FB2,
    #[doc = "0x270 - Filter bank 6 filtrate bit register 1"]
    pub f6fb1: F6FB1,
    #[doc = "0x274 - Filter bank 6 filtrate bit register 2"]
    pub f6fb2: F6FB2,
    #[doc = "0x278 - Filter bank 7 filtrate bit register 1"]
    pub f7fb1: F7FB1,
    #[doc = "0x27c - Filter bank 7 filtrate bit register 2"]
    pub f7fb2: F7FB2,
    #[doc = "0x280 - Filter bank 8 filtrate bit filtrate bit register 1"]
    pub f8fb1: F8FB1,
    #[doc = "0x284 - Filter bank 8 filtrate bit filtrate bit register 2"]
    pub f8fb2: F8FB2,
    #[doc = "0x288 - Filter bank 9 filtrate bit filtrate bit filtrate bit filtrate bit filtrate bit register 1"]
    pub f9fb1: F9FB1,
    #[doc = "0x28c - Filter bank 9 filtrate bit filtrate bit filtrate bit filtrate bit filtrate bit register 2"]
    pub f9fb2: F9FB2,
    #[doc = "0x290 - Filter bank 10 filtrate bit register 1"]
    pub f10fb1: F10FB1,
    #[doc = "0x294 - Filter bank 10 filtrate bit register 2"]
    pub f10fb2: F10FB2,
    #[doc = "0x298 - Filter bank 11 filtrate bit register 1"]
    pub f11fb1: F11FB1,
    #[doc = "0x29c - Filter bank 11 filtrate bit register 2"]
    pub f11fb2: F11FB2,
    #[doc = "0x2a0 - Filter bank 12 filtrate bit filtrate bit register 1"]
    pub f12fb1: F12FB1,
    #[doc = "0x2a4 - Filter bank 12 filtrate bit filtrate bit register 2"]
    pub f12fb2: F12FB2,
    #[doc = "0x2a8 - Filter bank 13 filtrate bit filtrate bit register 1"]
    pub f13fb1: F13FB1,
    #[doc = "0x2ac - Filter bank 13 filtrate bit filtrate bit register 2"]
    pub f13fb2: F13FB2,
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
#[doc = "RF0 (rw) register accessor: Receive FIFO 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rf0`]
module"]
pub type RF0 = crate::Reg<rf0::RF0_SPEC>;
#[doc = "Receive FIFO 0 register"]
pub mod rf0;
#[doc = "RF1 (rw) register accessor: Receive FIFO 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rf1`]
module"]
pub type RF1 = crate::Reg<rf1::RF1_SPEC>;
#[doc = "Receive FIFO 1 register"]
pub mod rf1;
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
#[doc = "TMI0 (rw) register accessor: Transmit mailbox 0 identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmi0`]
module"]
pub type TMI0 = crate::Reg<tmi0::TMI0_SPEC>;
#[doc = "Transmit mailbox 0 identifier register"]
pub mod tmi0;
#[doc = "TMC0 (rw) register accessor: Transmit mailbox 0 data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmc0`]
module"]
pub type TMC0 = crate::Reg<tmc0::TMC0_SPEC>;
#[doc = "Transmit mailbox 0 data length and time stamp register"]
pub mod tmc0;
#[doc = "TMDTL0 (rw) register accessor: Transmit mailbox 0 low byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdtl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdtl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmdtl0`]
module"]
pub type TMDTL0 = crate::Reg<tmdtl0::TMDTL0_SPEC>;
#[doc = "Transmit mailbox 0 low byte data register"]
pub mod tmdtl0;
#[doc = "TMDTH0 (rw) register accessor: Transmit mailbox 0 high byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdth0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdth0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmdth0`]
module"]
pub type TMDTH0 = crate::Reg<tmdth0::TMDTH0_SPEC>;
#[doc = "Transmit mailbox 0 high byte data register"]
pub mod tmdth0;
#[doc = "TMI1 (rw) register accessor: Transmit mailbox 1 identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmi1`]
module"]
pub type TMI1 = crate::Reg<tmi1::TMI1_SPEC>;
#[doc = "Transmit mailbox 1 identifier register"]
pub mod tmi1;
#[doc = "TMC1 (rw) register accessor: Transmit mailbox 1 data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmc1`]
module"]
pub type TMC1 = crate::Reg<tmc1::TMC1_SPEC>;
#[doc = "Transmit mailbox 1 data length and time stamp register"]
pub mod tmc1;
#[doc = "TMDTL1 (rw) register accessor: Transmit mailbox 1 low byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdtl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdtl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmdtl1`]
module"]
pub type TMDTL1 = crate::Reg<tmdtl1::TMDTL1_SPEC>;
#[doc = "Transmit mailbox 1 low byte data register"]
pub mod tmdtl1;
#[doc = "TMDTH1 (rw) register accessor: Transmit mailbox 1 high byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdth1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdth1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmdth1`]
module"]
pub type TMDTH1 = crate::Reg<tmdth1::TMDTH1_SPEC>;
#[doc = "Transmit mailbox 1 high byte data register"]
pub mod tmdth1;
#[doc = "TMI2 (rw) register accessor: Transmit mailbox 2 identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmi2`]
module"]
pub type TMI2 = crate::Reg<tmi2::TMI2_SPEC>;
#[doc = "Transmit mailbox 2 identifier register"]
pub mod tmi2;
#[doc = "TMC2 (rw) register accessor: Transmit mailbox 2 data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmc2`]
module"]
pub type TMC2 = crate::Reg<tmc2::TMC2_SPEC>;
#[doc = "Transmit mailbox 2 data length and time stamp register"]
pub mod tmc2;
#[doc = "TMDTL2 (rw) register accessor: Transmit mailbox 2 low byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdtl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdtl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmdtl2`]
module"]
pub type TMDTL2 = crate::Reg<tmdtl2::TMDTL2_SPEC>;
#[doc = "Transmit mailbox 2 low byte data register"]
pub mod tmdtl2;
#[doc = "TMDTH2 (rw) register accessor: Transmit mailbox 2 high byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdth2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdth2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tmdth2`]
module"]
pub type TMDTH2 = crate::Reg<tmdth2::TMDTH2_SPEC>;
#[doc = "Transmit mailbox 2 high byte data register"]
pub mod tmdth2;
#[doc = "RFI0 (r) register accessor: Receive FIFO 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfi0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfi0`]
module"]
pub type RFI0 = crate::Reg<rfi0::RFI0_SPEC>;
#[doc = "Receive FIFO 0 register"]
pub mod rfi0;
#[doc = "RFC0 (r) register accessor: Receive FIFO 0 data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfc0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfc0`]
module"]
pub type RFC0 = crate::Reg<rfc0::RFC0_SPEC>;
#[doc = "Receive FIFO 0 data length and time stamp register"]
pub mod rfc0;
#[doc = "RFDTL0 (r) register accessor: Receive FIFO 0 low byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdtl0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfdtl0`]
module"]
pub type RFDTL0 = crate::Reg<rfdtl0::RFDTL0_SPEC>;
#[doc = "Receive FIFO 0 low byte data register"]
pub mod rfdtl0;
#[doc = "RFDTH0 (r) register accessor: Receive FIFO 0 high byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdth0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfdth0`]
module"]
pub type RFDTH0 = crate::Reg<rfdth0::RFDTH0_SPEC>;
#[doc = "Receive FIFO 0 high byte data register"]
pub mod rfdth0;
#[doc = "RFI1 (r) register accessor: Receive FIFO 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfi1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfi1`]
module"]
pub type RFI1 = crate::Reg<rfi1::RFI1_SPEC>;
#[doc = "Receive FIFO 1 register"]
pub mod rfi1;
#[doc = "RFC1 (r) register accessor: Receive FIFO 1 data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfc1`]
module"]
pub type RFC1 = crate::Reg<rfc1::RFC1_SPEC>;
#[doc = "Receive FIFO 1 data length and time stamp register"]
pub mod rfc1;
#[doc = "RFDTL1 (r) register accessor: Receive FIFO 1 low byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdtl1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfdtl1`]
module"]
pub type RFDTL1 = crate::Reg<rfdtl1::RFDTL1_SPEC>;
#[doc = "Receive FIFO 1 low byte data register"]
pub mod rfdtl1;
#[doc = "RFDTH1 (r) register accessor: Receive FIFO 1 high byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdth1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfdth1`]
module"]
pub type RFDTH1 = crate::Reg<rfdth1::RFDTH1_SPEC>;
#[doc = "Receive FIFO 1 high byte data register"]
pub mod rfdth1;
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
#[doc = "F0FB1 (rw) register accessor: Filter bank 0 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f0fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f0fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f0fb1`]
module"]
pub type F0FB1 = crate::Reg<f0fb1::F0FB1_SPEC>;
#[doc = "Filter bank 0 filtrate bit register 1"]
pub mod f0fb1;
#[doc = "F0FB2 (rw) register accessor: Filter bank 0 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f0fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f0fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f0fb2`]
module"]
pub type F0FB2 = crate::Reg<f0fb2::F0FB2_SPEC>;
#[doc = "Filter bank 0 filtrate bit register 2"]
pub mod f0fb2;
#[doc = "F1FB1 (rw) register accessor: Filter bank 1 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f1fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f1fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f1fb1`]
module"]
pub type F1FB1 = crate::Reg<f1fb1::F1FB1_SPEC>;
#[doc = "Filter bank 1 filtrate bit register 1"]
pub mod f1fb1;
#[doc = "F1FB2 (rw) register accessor: Filter bank 1 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f1fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f1fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f1fb2`]
module"]
pub type F1FB2 = crate::Reg<f1fb2::F1FB2_SPEC>;
#[doc = "Filter bank 1 filtrate bit register 2"]
pub mod f1fb2;
#[doc = "F2FB1 (rw) register accessor: Filter bank 2 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f2fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f2fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f2fb1`]
module"]
pub type F2FB1 = crate::Reg<f2fb1::F2FB1_SPEC>;
#[doc = "Filter bank 2 filtrate bit register 1"]
pub mod f2fb1;
#[doc = "F2FB2 (rw) register accessor: Filter bank 2 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f2fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f2fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f2fb2`]
module"]
pub type F2FB2 = crate::Reg<f2fb2::F2FB2_SPEC>;
#[doc = "Filter bank 2 filtrate bit register 2"]
pub mod f2fb2;
#[doc = "F3FB1 (rw) register accessor: Filter bank 3 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f3fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f3fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f3fb1`]
module"]
pub type F3FB1 = crate::Reg<f3fb1::F3FB1_SPEC>;
#[doc = "Filter bank 3 filtrate bit register 1"]
pub mod f3fb1;
#[doc = "F3FB2 (rw) register accessor: Filter bank 3 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f3fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f3fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f3fb2`]
module"]
pub type F3FB2 = crate::Reg<f3fb2::F3FB2_SPEC>;
#[doc = "Filter bank 3 filtrate bit register 2"]
pub mod f3fb2;
#[doc = "F4FB1 (rw) register accessor: Filter bank 4 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f4fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f4fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f4fb1`]
module"]
pub type F4FB1 = crate::Reg<f4fb1::F4FB1_SPEC>;
#[doc = "Filter bank 4 filtrate bit register 1"]
pub mod f4fb1;
#[doc = "F4FB2 (rw) register accessor: Filter bank 4 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f4fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f4fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f4fb2`]
module"]
pub type F4FB2 = crate::Reg<f4fb2::F4FB2_SPEC>;
#[doc = "Filter bank 4 filtrate bit register 2"]
pub mod f4fb2;
#[doc = "F5FB1 (rw) register accessor: Filter bank 5 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f5fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f5fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f5fb1`]
module"]
pub type F5FB1 = crate::Reg<f5fb1::F5FB1_SPEC>;
#[doc = "Filter bank 5 filtrate bit register 1"]
pub mod f5fb1;
#[doc = "F5FB2 (rw) register accessor: Filter bank 5 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f5fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f5fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f5fb2`]
module"]
pub type F5FB2 = crate::Reg<f5fb2::F5FB2_SPEC>;
#[doc = "Filter bank 5 filtrate bit register 2"]
pub mod f5fb2;
#[doc = "F6FB1 (rw) register accessor: Filter bank 6 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f6fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f6fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f6fb1`]
module"]
pub type F6FB1 = crate::Reg<f6fb1::F6FB1_SPEC>;
#[doc = "Filter bank 6 filtrate bit register 1"]
pub mod f6fb1;
#[doc = "F6FB2 (rw) register accessor: Filter bank 6 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f6fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f6fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f6fb2`]
module"]
pub type F6FB2 = crate::Reg<f6fb2::F6FB2_SPEC>;
#[doc = "Filter bank 6 filtrate bit register 2"]
pub mod f6fb2;
#[doc = "F7FB1 (rw) register accessor: Filter bank 7 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f7fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f7fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f7fb1`]
module"]
pub type F7FB1 = crate::Reg<f7fb1::F7FB1_SPEC>;
#[doc = "Filter bank 7 filtrate bit register 1"]
pub mod f7fb1;
#[doc = "F7FB2 (rw) register accessor: Filter bank 7 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f7fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f7fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f7fb2`]
module"]
pub type F7FB2 = crate::Reg<f7fb2::F7FB2_SPEC>;
#[doc = "Filter bank 7 filtrate bit register 2"]
pub mod f7fb2;
#[doc = "F8FB1 (rw) register accessor: Filter bank 8 filtrate bit filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f8fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f8fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f8fb1`]
module"]
pub type F8FB1 = crate::Reg<f8fb1::F8FB1_SPEC>;
#[doc = "Filter bank 8 filtrate bit filtrate bit register 1"]
pub mod f8fb1;
#[doc = "F8FB2 (rw) register accessor: Filter bank 8 filtrate bit filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f8fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f8fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f8fb2`]
module"]
pub type F8FB2 = crate::Reg<f8fb2::F8FB2_SPEC>;
#[doc = "Filter bank 8 filtrate bit filtrate bit register 2"]
pub mod f8fb2;
#[doc = "F9FB1 (rw) register accessor: Filter bank 9 filtrate bit filtrate bit filtrate bit filtrate bit filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f9fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f9fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f9fb1`]
module"]
pub type F9FB1 = crate::Reg<f9fb1::F9FB1_SPEC>;
#[doc = "Filter bank 9 filtrate bit filtrate bit filtrate bit filtrate bit filtrate bit register 1"]
pub mod f9fb1;
#[doc = "F9FB2 (rw) register accessor: Filter bank 9 filtrate bit filtrate bit filtrate bit filtrate bit filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f9fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f9fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f9fb2`]
module"]
pub type F9FB2 = crate::Reg<f9fb2::F9FB2_SPEC>;
#[doc = "Filter bank 9 filtrate bit filtrate bit filtrate bit filtrate bit filtrate bit register 2"]
pub mod f9fb2;
#[doc = "F10FB1 (rw) register accessor: Filter bank 10 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f10fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f10fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f10fb1`]
module"]
pub type F10FB1 = crate::Reg<f10fb1::F10FB1_SPEC>;
#[doc = "Filter bank 10 filtrate bit register 1"]
pub mod f10fb1;
#[doc = "F10FB2 (rw) register accessor: Filter bank 10 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f10fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f10fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f10fb2`]
module"]
pub type F10FB2 = crate::Reg<f10fb2::F10FB2_SPEC>;
#[doc = "Filter bank 10 filtrate bit register 2"]
pub mod f10fb2;
#[doc = "F11FB1 (rw) register accessor: Filter bank 11 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f11fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f11fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f11fb1`]
module"]
pub type F11FB1 = crate::Reg<f11fb1::F11FB1_SPEC>;
#[doc = "Filter bank 11 filtrate bit register 1"]
pub mod f11fb1;
#[doc = "F11FB2 (rw) register accessor: Filter bank 11 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f11fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f11fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f11fb2`]
module"]
pub type F11FB2 = crate::Reg<f11fb2::F11FB2_SPEC>;
#[doc = "Filter bank 11 filtrate bit register 2"]
pub mod f11fb2;
#[doc = "F12FB1 (rw) register accessor: Filter bank 12 filtrate bit filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f12fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f12fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f12fb1`]
module"]
pub type F12FB1 = crate::Reg<f12fb1::F12FB1_SPEC>;
#[doc = "Filter bank 12 filtrate bit filtrate bit register 1"]
pub mod f12fb1;
#[doc = "F12FB2 (rw) register accessor: Filter bank 12 filtrate bit filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f12fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f12fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f12fb2`]
module"]
pub type F12FB2 = crate::Reg<f12fb2::F12FB2_SPEC>;
#[doc = "Filter bank 12 filtrate bit filtrate bit register 2"]
pub mod f12fb2;
#[doc = "F13FB1 (rw) register accessor: Filter bank 13 filtrate bit filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f13fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f13fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f13fb1`]
module"]
pub type F13FB1 = crate::Reg<f13fb1::F13FB1_SPEC>;
#[doc = "Filter bank 13 filtrate bit filtrate bit register 1"]
pub mod f13fb1;
#[doc = "F13FB2 (rw) register accessor: Filter bank 13 filtrate bit filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f13fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f13fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`f13fb2`]
module"]
pub type F13FB2 = crate::Reg<f13fb2::F13FB2_SPEC>;
#[doc = "Filter bank 13 filtrate bit filtrate bit register 2"]
pub mod f13fb2;
