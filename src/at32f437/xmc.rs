#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    pub bk1ctrl1: BK1CTRL1,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    pub bk1tmg1: BK1TMG1,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    pub bk1ctrl2: BK1CTRL2,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2"]
    pub bk1tmg2: BK1TMG2,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 3"]
    pub bk1ctrl3: BK1CTRL3,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3"]
    pub bk1tmg3: BK1TMG3,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    pub bk1ctrl4: BK1CTRL4,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    pub bk1tmg4: BK1TMG4,
    _reserved8: [u8; 0x40],
    #[doc = "0x60 - PC Card/NAND Flash control register 2"]
    pub bk2ctrl: BK2CTRL,
    #[doc = "0x64 - FIFO status and interrupt register 2"]
    pub bk2is: BK2IS,
    #[doc = "0x68 - Regular memory space timing register 2"]
    pub bk2tmgrg: BK2TMGRG,
    #[doc = "0x6c - special memory space timing register 2"]
    pub bk2tmgsp: BK2TMGSP,
    _reserved12: [u8; 0x04],
    #[doc = "0x74 - ECC result register 2"]
    pub bk2ecc: BK2ECC,
    _reserved13: [u8; 0x08],
    #[doc = "0x80 - PC Card/NAND Flash control register 3"]
    pub bk3ctrl: BK3CTRL,
    #[doc = "0x84 - FIFO status and interrupt register 3"]
    pub bk3is: BK3IS,
    #[doc = "0x88 - Regular memory space timing register 3"]
    pub bk3tmgrg: BK3TMGRG,
    #[doc = "0x8c - special memory space timing register 3"]
    pub bk3tmgsp: BK3TMGSP,
    _reserved17: [u8; 0x04],
    #[doc = "0x94 - ECC result register 3"]
    pub bk3ecc: BK3ECC,
    _reserved18: [u8; 0x08],
    #[doc = "0xa0 - PC Card/NAND Flash control register 4"]
    pub bk4ctrl: BK4CTRL,
    #[doc = "0xa4 - FIFO status and interrupt register 4"]
    pub bk4is: BK4IS,
    #[doc = "0xa8 - Regular memory space timing register 4"]
    pub bk4tmgcm: BK4TMGCM,
    #[doc = "0xac - special memory space timing register 4"]
    pub bk4tmgat: BK4TMGAT,
    #[doc = "0xb0 - I/O space timing register 4"]
    pub bk4tmgio: BK4TMGIO,
    _reserved23: [u8; 0x50],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bk1tmgwr1: BK1TMGWR1,
    _reserved24: [u8; 0x04],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"]
    pub bk1tmgwr2: BK1TMGWR2,
    _reserved25: [u8; 0x04],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"]
    pub bk1tmgwr3: BK1TMGWR3,
    _reserved26: [u8; 0x04],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    pub bk1tmgwr4: BK1TMGWR4,
    _reserved27: [u8; 0x20],
    #[doc = "0x140 - SDRAM Control Register 1"]
    pub ctrl1: CTRL1,
    #[doc = "0x144 - SDRAM Control Register 2"]
    pub ctrl2: CTRL2,
    #[doc = "0x148 - SDRAM Timing register 1"]
    pub tm1: TM1,
    #[doc = "0x14c - SDRAM Timing register 2"]
    pub tm2: TM2,
    #[doc = "0x150 - SDRAM Command Mode register"]
    pub cmd: CMD,
    #[doc = "0x154 - SDRAM Refresh Timer register"]
    pub rcnt: RCNT,
    #[doc = "0x158 - SDRAM Status register"]
    pub sts: STS,
    _reserved34: [u8; 0xc4],
    #[doc = "0x220 - externl timeing register 1"]
    pub ext1: EXT1,
    #[doc = "0x224 - externl timeing register 2"]
    pub ext2: EXT2,
    #[doc = "0x228 - externl timeing register 3"]
    pub ext3: EXT3,
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
#[doc = "BK1CTRL2 (rw) register accessor: SRAM/NOR-Flash chip-select control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk1ctrl2`]
module"]
pub type BK1CTRL2 = crate::Reg<bk1ctrl2::BK1CTRL2_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bk1ctrl2;
#[doc = "BK1TMG2 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk1tmg2`]
module"]
pub type BK1TMG2 = crate::Reg<bk1tmg2::BK1TMG2_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub mod bk1tmg2;
#[doc = "BK1CTRL3 (rw) register accessor: SRAM/NOR-Flash chip-select control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk1ctrl3`]
module"]
pub type BK1CTRL3 = crate::Reg<bk1ctrl3::BK1CTRL3_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub mod bk1ctrl3;
#[doc = "BK1TMG3 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk1tmg3`]
module"]
pub type BK1TMG3 = crate::Reg<bk1tmg3::BK1TMG3_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub mod bk1tmg3;
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
#[doc = "BK2TMGRG (rw) register accessor: Regular memory space timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2tmgrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2tmgrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk2tmgrg`]
module"]
pub type BK2TMGRG = crate::Reg<bk2tmgrg::BK2TMGRG_SPEC>;
#[doc = "Regular memory space timing register 2"]
pub mod bk2tmgrg;
#[doc = "BK2TMGSP (rw) register accessor: special memory space timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2tmgsp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2tmgsp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk2tmgsp`]
module"]
pub type BK2TMGSP = crate::Reg<bk2tmgsp::BK2TMGSP_SPEC>;
#[doc = "special memory space timing register 2"]
pub mod bk2tmgsp;
#[doc = "BK2ECC (rw) register accessor: ECC result register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2ecc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2ecc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk2ecc`]
module"]
pub type BK2ECC = crate::Reg<bk2ecc::BK2ECC_SPEC>;
#[doc = "ECC result register 2"]
pub mod bk2ecc;
#[doc = "BK3CTRL (rw) register accessor: PC Card/NAND Flash control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk3ctrl`]
module"]
pub type BK3CTRL = crate::Reg<bk3ctrl::BK3CTRL_SPEC>;
#[doc = "PC Card/NAND Flash control register 3"]
pub mod bk3ctrl;
#[doc = "BK3IS (rw) register accessor: FIFO status and interrupt register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk3is`]
module"]
pub type BK3IS = crate::Reg<bk3is::BK3IS_SPEC>;
#[doc = "FIFO status and interrupt register 3"]
pub mod bk3is;
#[doc = "BK3TMGRG (rw) register accessor: Regular memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3tmgrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3tmgrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk3tmgrg`]
module"]
pub type BK3TMGRG = crate::Reg<bk3tmgrg::BK3TMGRG_SPEC>;
#[doc = "Regular memory space timing register 3"]
pub mod bk3tmgrg;
#[doc = "BK3TMGSP (rw) register accessor: special memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3tmgsp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3tmgsp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk3tmgsp`]
module"]
pub type BK3TMGSP = crate::Reg<bk3tmgsp::BK3TMGSP_SPEC>;
#[doc = "special memory space timing register 3"]
pub mod bk3tmgsp;
#[doc = "BK3ECC (rw) register accessor: ECC result register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3ecc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3ecc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk3ecc`]
module"]
pub type BK3ECC = crate::Reg<bk3ecc::BK3ECC_SPEC>;
#[doc = "ECC result register 3"]
pub mod bk3ecc;
#[doc = "BK4CTRL (rw) register accessor: PC Card/NAND Flash control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk4ctrl`]
module"]
pub type BK4CTRL = crate::Reg<bk4ctrl::BK4CTRL_SPEC>;
#[doc = "PC Card/NAND Flash control register 4"]
pub mod bk4ctrl;
#[doc = "BK4IS (rw) register accessor: FIFO status and interrupt register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk4is`]
module"]
pub type BK4IS = crate::Reg<bk4is::BK4IS_SPEC>;
#[doc = "FIFO status and interrupt register 4"]
pub mod bk4is;
#[doc = "BK4TMGCM (rw) register accessor: Regular memory space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4tmgcm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4tmgcm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk4tmgcm`]
module"]
pub type BK4TMGCM = crate::Reg<bk4tmgcm::BK4TMGCM_SPEC>;
#[doc = "Regular memory space timing register 4"]
pub mod bk4tmgcm;
#[doc = "BK4TMGAT (rw) register accessor: special memory space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4tmgat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4tmgat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk4tmgat`]
module"]
pub type BK4TMGAT = crate::Reg<bk4tmgat::BK4TMGAT_SPEC>;
#[doc = "special memory space timing register 4"]
pub mod bk4tmgat;
#[doc = "BK4TMGIO (rw) register accessor: I/O space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4tmgio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4tmgio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk4tmgio`]
module"]
pub type BK4TMGIO = crate::Reg<bk4tmgio::BK4TMGIO_SPEC>;
#[doc = "I/O space timing register 4"]
pub mod bk4tmgio;
#[doc = "BK1TMGWR1 (rw) register accessor: SRAM/NOR-Flash write timing registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmgwr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmgwr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk1tmgwr1`]
module"]
pub type BK1TMGWR1 = crate::Reg<bk1tmgwr1::BK1TMGWR1_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bk1tmgwr1;
#[doc = "BK1TMGWR2 (rw) register accessor: SRAM/NOR-Flash write timing registers 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmgwr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmgwr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk1tmgwr2`]
module"]
pub type BK1TMGWR2 = crate::Reg<bk1tmgwr2::BK1TMGWR2_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub mod bk1tmgwr2;
#[doc = "BK1TMGWR3 (rw) register accessor: SRAM/NOR-Flash write timing registers 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmgwr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmgwr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk1tmgwr3`]
module"]
pub type BK1TMGWR3 = crate::Reg<bk1tmgwr3::BK1TMGWR3_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub mod bk1tmgwr3;
#[doc = "BK1TMGWR4 (rw) register accessor: SRAM/NOR-Flash write timing registers 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmgwr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmgwr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bk1tmgwr4`]
module"]
pub type BK1TMGWR4 = crate::Reg<bk1tmgwr4::BK1TMGWR4_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bk1tmgwr4;
#[doc = "CTRL1 (rw) register accessor: SDRAM Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "SDRAM Control Register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: SDRAM Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "SDRAM Control Register 2"]
pub mod ctrl2;
#[doc = "TM1 (rw) register accessor: SDRAM Timing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tm1`]
module"]
pub type TM1 = crate::Reg<tm1::TM1_SPEC>;
#[doc = "SDRAM Timing register 1"]
pub mod tm1;
#[doc = "TM2 (rw) register accessor: SDRAM Timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tm2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tm2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tm2`]
module"]
pub type TM2 = crate::Reg<tm2::TM2_SPEC>;
#[doc = "SDRAM Timing register 2"]
pub mod tm2;
#[doc = "CMD (rw) register accessor: SDRAM Command Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "SDRAM Command Mode register"]
pub mod cmd;
#[doc = "RCNT (rw) register accessor: SDRAM Refresh Timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rcnt`]
module"]
pub type RCNT = crate::Reg<rcnt::RCNT_SPEC>;
#[doc = "SDRAM Refresh Timer register"]
pub mod rcnt;
#[doc = "STS (r) register accessor: SDRAM Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "SDRAM Status register"]
pub mod sts;
#[doc = "EXT1 (rw) register accessor: externl timeing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext1`]
module"]
pub type EXT1 = crate::Reg<ext1::EXT1_SPEC>;
#[doc = "externl timeing register 1"]
pub mod ext1;
#[doc = "EXT2 (rw) register accessor: externl timeing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext2`]
module"]
pub type EXT2 = crate::Reg<ext2::EXT2_SPEC>;
#[doc = "externl timeing register 2"]
pub mod ext2;
#[doc = "EXT3 (rw) register accessor: externl timeing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext3`]
module"]
pub type EXT3 = crate::Reg<ext3::EXT3_SPEC>;
#[doc = "externl timeing register 3"]
pub mod ext3;
#[doc = "EXT4 (rw) register accessor: externl timeing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ext4`]
module"]
pub type EXT4 = crate::Reg<ext4::EXT4_SPEC>;
#[doc = "externl timeing register 4"]
pub mod ext4;
