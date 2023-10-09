#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    pub bk1ctrl1: BK1CTRL1,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    pub bk1tmg1: BK1TMG1,
    _reserved2: [u8; 0x10],
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    pub bk1ctrl4: BK1CTRL4,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    pub bk1tmg4: BK1TMG4,
    _reserved4: [u8; 0x40],
    #[doc = "0x60 - PC Card/NAND Flash control register 2"]
    pub bk2ctrl: BK2CTRL,
    #[doc = "0x64 - FIFO status and interrupt register 2"]
    pub bk2is: BK2IS,
    #[doc = "0x68 - Regular memory space timing register 2"]
    pub bk2tmgmem: BK2TMGMEM,
    #[doc = "0x6c - special memory space timing register 2"]
    pub bk2tmgatt: BK2TMGATT,
    _reserved8: [u8; 0x04],
    #[doc = "0x74 - ECC result register 2"]
    pub bk2ecc: BK2ECC,
    _reserved9: [u8; 0x8c],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bk1tmgwr1: BK1TMGWR1,
    _reserved10: [u8; 0x14],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    pub bk1tmgwr4: BK1TMGWR4,
    _reserved11: [u8; 0x0100],
    #[doc = "0x220 - externl timeing register 1"]
    pub ext1: EXT1,
    _reserved12: [u8; 0x08],
    #[doc = "0x22c - externl timeing register 4"]
    pub ext4: EXT4,
}
#[doc = "BK1CTRL1 (rw) register accessor: SRAM/NOR-Flash chip-select control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk1ctrl1`]
module"]
pub type BK1CTRL1 = crate::Reg<bk1ctrl1::BK1CTRL1_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bk1ctrl1;
#[doc = "BK1TMG1 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk1tmg1`]
module"]
pub type BK1TMG1 = crate::Reg<bk1tmg1::BK1TMG1_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod bk1tmg1;
#[doc = "BK1CTRL4 (rw) register accessor: SRAM/NOR-Flash chip-select control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1ctrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1ctrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk1ctrl4`]
module"]
pub type BK1CTRL4 = crate::Reg<bk1ctrl4::BK1CTRL4_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub mod bk1ctrl4;
#[doc = "BK1TMG4 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk1tmg4`]
module"]
pub type BK1TMG4 = crate::Reg<bk1tmg4::BK1TMG4_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub mod bk1tmg4;
#[doc = "BK2CTRL (rw) register accessor: PC Card/NAND Flash control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk2ctrl`]
module"]
pub type BK2CTRL = crate::Reg<bk2ctrl::BK2CTRL_SPEC>;
#[doc = "PC Card/NAND Flash control register 2"]
pub mod bk2ctrl;
#[doc = "BK2IS (rw) register accessor: FIFO status and interrupt register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk2is`]
module"]
pub type BK2IS = crate::Reg<bk2is::BK2IS_SPEC>;
#[doc = "FIFO status and interrupt register 2"]
pub mod bk2is;
#[doc = "BK2TMGMEM (rw) register accessor: Regular memory space timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2tmgmem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2tmgmem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk2tmgmem`]
module"]
pub type BK2TMGMEM = crate::Reg<bk2tmgmem::BK2TMGMEM_SPEC>;
#[doc = "Regular memory space timing register 2"]
pub mod bk2tmgmem;
#[doc = "BK2TMGATT (rw) register accessor: special memory space timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2tmgatt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2tmgatt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk2tmgatt`]
module"]
pub type BK2TMGATT = crate::Reg<bk2tmgatt::BK2TMGATT_SPEC>;
#[doc = "special memory space timing register 2"]
pub mod bk2tmgatt;
#[doc = "BK2ECC (rw) register accessor: ECC result register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2ecc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2ecc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk2ecc`]
module"]
pub type BK2ECC = crate::Reg<bk2ecc::BK2ECC_SPEC>;
#[doc = "ECC result register 2"]
pub mod bk2ecc;
#[doc = "BK1TMGWR1 (rw) register accessor: SRAM/NOR-Flash write timing registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmgwr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmgwr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk1tmgwr1`]
module"]
pub type BK1TMGWR1 = crate::Reg<bk1tmgwr1::BK1TMGWR1_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bk1tmgwr1;
#[doc = "BK1TMGWR4 (rw) register accessor: SRAM/NOR-Flash write timing registers 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmgwr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmgwr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk1tmgwr4`]
module"]
pub type BK1TMGWR4 = crate::Reg<bk1tmgwr4::BK1TMGWR4_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bk1tmgwr4;
#[doc = "EXT1 (rw) register accessor: externl timeing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext1`]
module"]
pub type EXT1 = crate::Reg<ext1::EXT1_SPEC>;
#[doc = "externl timeing register 1"]
pub mod ext1;
#[doc = "EXT4 (rw) register accessor: externl timeing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext4`]
module"]
pub type EXT4 = crate::Reg<ext4::EXT4_SPEC>;
#[doc = "externl timeing register 4"]
pub mod ext4;
