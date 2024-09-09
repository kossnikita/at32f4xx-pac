#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bk1ctrl1: BK1CTRL1,
    bk1tmg1: BK1TMG1,
    bk1ctrl2: BK1CTRL2,
    bk1tmg2: BK1TMG2,
    bk1ctrl3: BK1CTRL3,
    bk1tmg3: BK1TMG3,
    bk1ctrl4: BK1CTRL4,
    bk1tmg4: BK1TMG4,
    _reserved8: [u8; 0x40],
    bk2ctrl: BK2CTRL,
    bk2is: BK2IS,
    bk2tmgmem: BK2TMGMEM,
    bk2tmgatt: BK2TMGATT,
    _reserved12: [u8; 0x04],
    bk2ecc: BK2ECC,
    _reserved13: [u8; 0x08],
    bk3ctrl: BK3CTRL,
    bk3is: BK3IS,
    bk3tmgmem: BK3TMGMEM,
    bk3tmgatt: BK3TMGATT,
    _reserved17: [u8; 0x04],
    bk3ecc: BK3ECC,
    _reserved18: [u8; 0x08],
    bk4ctrl: BK4CTRL,
    bk4is: BK4IS,
    bk4tmgmem: BK4TMGMEM,
    bk4tmgatt: BK4TMGATT,
    bk4tmgio: BK4TMGIO,
    _reserved23: [u8; 0x50],
    bk1tmgwr1: BK1TMGWR1,
    _reserved24: [u8; 0x04],
    bk1tmgwr2: BK1TMGWR2,
    _reserved25: [u8; 0x04],
    bk1tmgwr3: BK1TMGWR3,
    _reserved26: [u8; 0x04],
    bk1tmgwr4: BK1TMGWR4,
    _reserved27: [u8; 0x0100],
    ext1: EXT1,
    ext2: EXT2,
    ext3: EXT3,
    ext4: EXT4,
}
impl RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    #[inline(always)]
    pub const fn bk1ctrl1(&self) -> &BK1CTRL1 {
        &self.bk1ctrl1
    }
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    #[inline(always)]
    pub const fn bk1tmg1(&self) -> &BK1TMG1 {
        &self.bk1tmg1
    }
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    #[inline(always)]
    pub const fn bk1ctrl2(&self) -> &BK1CTRL2 {
        &self.bk1ctrl2
    }
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2"]
    #[inline(always)]
    pub const fn bk1tmg2(&self) -> &BK1TMG2 {
        &self.bk1tmg2
    }
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 3"]
    #[inline(always)]
    pub const fn bk1ctrl3(&self) -> &BK1CTRL3 {
        &self.bk1ctrl3
    }
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3"]
    #[inline(always)]
    pub const fn bk1tmg3(&self) -> &BK1TMG3 {
        &self.bk1tmg3
    }
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    #[inline(always)]
    pub const fn bk1ctrl4(&self) -> &BK1CTRL4 {
        &self.bk1ctrl4
    }
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    #[inline(always)]
    pub const fn bk1tmg4(&self) -> &BK1TMG4 {
        &self.bk1tmg4
    }
    #[doc = "0x60 - PC Card/NAND Flash control register 2"]
    #[inline(always)]
    pub const fn bk2ctrl(&self) -> &BK2CTRL {
        &self.bk2ctrl
    }
    #[doc = "0x64 - FIFO status and interrupt register 2"]
    #[inline(always)]
    pub const fn bk2is(&self) -> &BK2IS {
        &self.bk2is
    }
    #[doc = "0x68 - Regular memory space timing register 2"]
    #[inline(always)]
    pub const fn bk2tmgmem(&self) -> &BK2TMGMEM {
        &self.bk2tmgmem
    }
    #[doc = "0x6c - special memory space timing register 2"]
    #[inline(always)]
    pub const fn bk2tmgatt(&self) -> &BK2TMGATT {
        &self.bk2tmgatt
    }
    #[doc = "0x74 - ECC result register 2"]
    #[inline(always)]
    pub const fn bk2ecc(&self) -> &BK2ECC {
        &self.bk2ecc
    }
    #[doc = "0x80 - PC Card/NAND Flash control register 3"]
    #[inline(always)]
    pub const fn bk3ctrl(&self) -> &BK3CTRL {
        &self.bk3ctrl
    }
    #[doc = "0x84 - FIFO status and interrupt register 3"]
    #[inline(always)]
    pub const fn bk3is(&self) -> &BK3IS {
        &self.bk3is
    }
    #[doc = "0x88 - Regular memory space timing register 3"]
    #[inline(always)]
    pub const fn bk3tmgmem(&self) -> &BK3TMGMEM {
        &self.bk3tmgmem
    }
    #[doc = "0x8c - special memory space timing register 3"]
    #[inline(always)]
    pub const fn bk3tmgatt(&self) -> &BK3TMGATT {
        &self.bk3tmgatt
    }
    #[doc = "0x94 - ECC result register 3"]
    #[inline(always)]
    pub const fn bk3ecc(&self) -> &BK3ECC {
        &self.bk3ecc
    }
    #[doc = "0xa0 - PC Card/NAND Flash control register 4"]
    #[inline(always)]
    pub const fn bk4ctrl(&self) -> &BK4CTRL {
        &self.bk4ctrl
    }
    #[doc = "0xa4 - FIFO status and interrupt register 4"]
    #[inline(always)]
    pub const fn bk4is(&self) -> &BK4IS {
        &self.bk4is
    }
    #[doc = "0xa8 - Regular memory space timing register 4"]
    #[inline(always)]
    pub const fn bk4tmgmem(&self) -> &BK4TMGMEM {
        &self.bk4tmgmem
    }
    #[doc = "0xac - special memory space timing register 4"]
    #[inline(always)]
    pub const fn bk4tmgatt(&self) -> &BK4TMGATT {
        &self.bk4tmgatt
    }
    #[doc = "0xb0 - special IO space timing register 4"]
    #[inline(always)]
    pub const fn bk4tmgio(&self) -> &BK4TMGIO {
        &self.bk4tmgio
    }
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    #[inline(always)]
    pub const fn bk1tmgwr1(&self) -> &BK1TMGWR1 {
        &self.bk1tmgwr1
    }
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"]
    #[inline(always)]
    pub const fn bk1tmgwr2(&self) -> &BK1TMGWR2 {
        &self.bk1tmgwr2
    }
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"]
    #[inline(always)]
    pub const fn bk1tmgwr3(&self) -> &BK1TMGWR3 {
        &self.bk1tmgwr3
    }
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    #[inline(always)]
    pub const fn bk1tmgwr4(&self) -> &BK1TMGWR4 {
        &self.bk1tmgwr4
    }
    #[doc = "0x220 - externl timeing register 1"]
    #[inline(always)]
    pub const fn ext1(&self) -> &EXT1 {
        &self.ext1
    }
    #[doc = "0x224 - externl timeing register 2"]
    #[inline(always)]
    pub const fn ext2(&self) -> &EXT2 {
        &self.ext2
    }
    #[doc = "0x228 - externl timeing register 3"]
    #[inline(always)]
    pub const fn ext3(&self) -> &EXT3 {
        &self.ext3
    }
    #[doc = "0x22c - externl timeing register 4"]
    #[inline(always)]
    pub const fn ext4(&self) -> &EXT4 {
        &self.ext4
    }
}
#[doc = "BK1CTRL1 (rw) register accessor: SRAM/NOR-Flash chip-select control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1ctrl1`]
module"]
pub type BK1CTRL1 = crate::Reg<bk1ctrl1::BK1CTRL1_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bk1ctrl1;
#[doc = "BK1TMG1 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1tmg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1tmg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmg1`]
module"]
pub type BK1TMG1 = crate::Reg<bk1tmg1::BK1TMG1_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod bk1tmg1;
#[doc = "BK1CTRL2 (rw) register accessor: SRAM/NOR-Flash chip-select control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1ctrl2`]
module"]
pub type BK1CTRL2 = crate::Reg<bk1ctrl2::BK1CTRL2_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bk1ctrl2;
#[doc = "BK1TMG2 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1tmg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1tmg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmg2`]
module"]
pub type BK1TMG2 = crate::Reg<bk1tmg2::BK1TMG2_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub mod bk1tmg2;
#[doc = "BK1CTRL3 (rw) register accessor: SRAM/NOR-Flash chip-select control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1ctrl3`]
module"]
pub type BK1CTRL3 = crate::Reg<bk1ctrl3::BK1CTRL3_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub mod bk1ctrl3;
#[doc = "BK1TMG3 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1tmg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1tmg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmg3`]
module"]
pub type BK1TMG3 = crate::Reg<bk1tmg3::BK1TMG3_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub mod bk1tmg3;
#[doc = "BK1CTRL4 (rw) register accessor: SRAM/NOR-Flash chip-select control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1ctrl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1ctrl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1ctrl4`]
module"]
pub type BK1CTRL4 = crate::Reg<bk1ctrl4::BK1CTRL4_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub mod bk1ctrl4;
#[doc = "BK1TMG4 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1tmg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1tmg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmg4`]
module"]
pub type BK1TMG4 = crate::Reg<bk1tmg4::BK1TMG4_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub mod bk1tmg4;
#[doc = "BK2CTRL (rw) register accessor: PC Card/NAND Flash control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk2ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk2ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2ctrl`]
module"]
pub type BK2CTRL = crate::Reg<bk2ctrl::BK2CTRL_SPEC>;
#[doc = "PC Card/NAND Flash control register 2"]
pub mod bk2ctrl;
#[doc = "BK2IS (rw) register accessor: FIFO status and interrupt register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk2is::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk2is::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2is`]
module"]
pub type BK2IS = crate::Reg<bk2is::BK2IS_SPEC>;
#[doc = "FIFO status and interrupt register 2"]
pub mod bk2is;
#[doc = "BK2TMGMEM (rw) register accessor: Regular memory space timing register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk2tmgmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk2tmgmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2tmgmem`]
module"]
pub type BK2TMGMEM = crate::Reg<bk2tmgmem::BK2TMGMEM_SPEC>;
#[doc = "Regular memory space timing register 2"]
pub mod bk2tmgmem;
#[doc = "BK2TMGATT (rw) register accessor: special memory space timing register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk2tmgatt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk2tmgatt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2tmgatt`]
module"]
pub type BK2TMGATT = crate::Reg<bk2tmgatt::BK2TMGATT_SPEC>;
#[doc = "special memory space timing register 2"]
pub mod bk2tmgatt;
#[doc = "BK2ECC (rw) register accessor: ECC result register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk2ecc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk2ecc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2ecc`]
module"]
pub type BK2ECC = crate::Reg<bk2ecc::BK2ECC_SPEC>;
#[doc = "ECC result register 2"]
pub mod bk2ecc;
#[doc = "BK3CTRL (rw) register accessor: PC Card/NAND Flash control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bk3ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk3ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk3ctrl`]
module"]
pub type BK3CTRL = crate::Reg<bk3ctrl::BK3CTRL_SPEC>;
#[doc = "PC Card/NAND Flash control register 3"]
pub mod bk3ctrl;
#[doc = "BK3IS (rw) register accessor: FIFO status and interrupt register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bk3is::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk3is::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk3is`]
module"]
pub type BK3IS = crate::Reg<bk3is::BK3IS_SPEC>;
#[doc = "FIFO status and interrupt register 3"]
pub mod bk3is;
#[doc = "BK3TMGMEM (rw) register accessor: Regular memory space timing register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bk3tmgmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk3tmgmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk3tmgmem`]
module"]
pub type BK3TMGMEM = crate::Reg<bk3tmgmem::BK3TMGMEM_SPEC>;
#[doc = "Regular memory space timing register 3"]
pub mod bk3tmgmem;
#[doc = "BK3TMGATT (rw) register accessor: special memory space timing register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bk3tmgatt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk3tmgatt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk3tmgatt`]
module"]
pub type BK3TMGATT = crate::Reg<bk3tmgatt::BK3TMGATT_SPEC>;
#[doc = "special memory space timing register 3"]
pub mod bk3tmgatt;
#[doc = "BK3ECC (rw) register accessor: ECC result register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bk3ecc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk3ecc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk3ecc`]
module"]
pub type BK3ECC = crate::Reg<bk3ecc::BK3ECC_SPEC>;
#[doc = "ECC result register 3"]
pub mod bk3ecc;
#[doc = "BK4CTRL (rw) register accessor: PC Card/NAND Flash control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`bk4ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk4ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk4ctrl`]
module"]
pub type BK4CTRL = crate::Reg<bk4ctrl::BK4CTRL_SPEC>;
#[doc = "PC Card/NAND Flash control register 4"]
pub mod bk4ctrl;
#[doc = "BK4IS (rw) register accessor: FIFO status and interrupt register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`bk4is::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk4is::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk4is`]
module"]
pub type BK4IS = crate::Reg<bk4is::BK4IS_SPEC>;
#[doc = "FIFO status and interrupt register 4"]
pub mod bk4is;
#[doc = "BK4TMGMEM (rw) register accessor: Regular memory space timing register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`bk4tmgmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk4tmgmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk4tmgmem`]
module"]
pub type BK4TMGMEM = crate::Reg<bk4tmgmem::BK4TMGMEM_SPEC>;
#[doc = "Regular memory space timing register 4"]
pub mod bk4tmgmem;
#[doc = "BK4TMGATT (rw) register accessor: special memory space timing register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`bk4tmgatt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk4tmgatt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk4tmgatt`]
module"]
pub type BK4TMGATT = crate::Reg<bk4tmgatt::BK4TMGATT_SPEC>;
#[doc = "special memory space timing register 4"]
pub mod bk4tmgatt;
#[doc = "BK4TMGIO (rw) register accessor: special IO space timing register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`bk4tmgio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk4tmgio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk4tmgio`]
module"]
pub type BK4TMGIO = crate::Reg<bk4tmgio::BK4TMGIO_SPEC>;
#[doc = "special IO space timing register 4"]
pub mod bk4tmgio;
#[doc = "BK1TMGWR1 (rw) register accessor: SRAM/NOR-Flash write timing registers 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1tmgwr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1tmgwr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmgwr1`]
module"]
pub type BK1TMGWR1 = crate::Reg<bk1tmgwr1::BK1TMGWR1_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bk1tmgwr1;
#[doc = "BK1TMGWR2 (rw) register accessor: SRAM/NOR-Flash write timing registers 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1tmgwr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1tmgwr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmgwr2`]
module"]
pub type BK1TMGWR2 = crate::Reg<bk1tmgwr2::BK1TMGWR2_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub mod bk1tmgwr2;
#[doc = "BK1TMGWR3 (rw) register accessor: SRAM/NOR-Flash write timing registers 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1tmgwr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1tmgwr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmgwr3`]
module"]
pub type BK1TMGWR3 = crate::Reg<bk1tmgwr3::BK1TMGWR3_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub mod bk1tmgwr3;
#[doc = "BK1TMGWR4 (rw) register accessor: SRAM/NOR-Flash write timing registers 4\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1tmgwr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1tmgwr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmgwr4`]
module"]
pub type BK1TMGWR4 = crate::Reg<bk1tmgwr4::BK1TMGWR4_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bk1tmgwr4;
#[doc = "EXT1 (rw) register accessor: externl timeing register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ext1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext1`]
module"]
pub type EXT1 = crate::Reg<ext1::EXT1_SPEC>;
#[doc = "externl timeing register 1"]
pub mod ext1;
#[doc = "EXT2 (rw) register accessor: externl timeing register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ext2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext2`]
module"]
pub type EXT2 = crate::Reg<ext2::EXT2_SPEC>;
#[doc = "externl timeing register 2"]
pub mod ext2;
#[doc = "EXT3 (rw) register accessor: externl timeing register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ext3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext3`]
module"]
pub type EXT3 = crate::Reg<ext3::EXT3_SPEC>;
#[doc = "externl timeing register 3"]
pub mod ext3;
#[doc = "EXT4 (rw) register accessor: externl timeing register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ext4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext4`]
module"]
pub type EXT4 = crate::Reg<ext4::EXT4_SPEC>;
#[doc = "externl timeing register 4"]
pub mod ext4;
