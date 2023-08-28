#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA status register (DMA_STS)"]
    pub sts: STS,
    #[doc = "0x04 - DMA flag clear register (DMA_CLR)"]
    pub clr: CLR,
    #[doc = "0x08 - DMA channel configuration register"]
    pub c1ctrl: C1CTRL,
    #[doc = "0x0c - DMA channel 1 number of data to transfer register"]
    pub c1dtcnt: C1DTCNT,
    #[doc = "0x10 - DMA channel 1 peripheral base address register"]
    pub c1paddr: C1PADDR,
    #[doc = "0x14 - DMA channel 1 memory base address register"]
    pub c1maddr: C1MADDR,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - DMA channel configuration register"]
    pub c2ctrl: C2CTRL,
    #[doc = "0x20 - DMA channel 2 number of data to transfer register"]
    pub c2dtcnt: C2DTCNT,
    #[doc = "0x24 - DMA channel 2 peripheral base address register"]
    pub c2paddr: C2PADDR,
    #[doc = "0x28 - DMA channel 2 memory base address register"]
    pub c2maddr: C2MADDR,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - DMA channel configuration register"]
    pub c3ctrl: C3CTRL,
    #[doc = "0x34 - DMA channel 3 number of data to transfer register"]
    pub c3dtcnt: C3DTCNT,
    #[doc = "0x38 - DMA channel 3 peripheral base address register"]
    pub c3paddr: C3PADDR,
    #[doc = "0x3c - DMA channel 3 memory base address register"]
    pub c3maddr: C3MADDR,
    _reserved14: [u8; 0x04],
    #[doc = "0x44 - DMA channel configuration register"]
    pub c4ctrl: C4CTRL,
    #[doc = "0x48 - DMA channel 4 number of data to transfer register"]
    pub c4dtcnt: C4DTCNT,
    #[doc = "0x4c - DMA channel 4 peripheral base address register"]
    pub c4paddr: C4PADDR,
    #[doc = "0x50 - DMA channel 4 memory base address register"]
    pub c4maddr: C4MADDR,
    _reserved18: [u8; 0x04],
    #[doc = "0x58 - DMA channel configuration register"]
    pub c5ctrl: C5CTRL,
    #[doc = "0x5c - DMA channel 5 number of data to transfer register"]
    pub c5dtcnt: C5DTCNT,
    #[doc = "0x60 - DMA channel 5 peripheral base address register"]
    pub c5paddr: C5PADDR,
    #[doc = "0x64 - DMA channel 5 memory base address register"]
    pub c5maddr: C5MADDR,
}
#[doc = "STS (r) register accessor: DMA status register (DMA_STS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "DMA status register (DMA_STS)"]
pub mod sts;
#[doc = "CLR (rw) register accessor: DMA flag clear register (DMA_CLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr`]
module"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "DMA flag clear register (DMA_CLR)"]
pub mod clr;
#[doc = "C1CTRL (rw) register accessor: DMA channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1ctrl`]
module"]
pub type C1CTRL = crate::Reg<c1ctrl::C1CTRL_SPEC>;
#[doc = "DMA channel configuration register"]
pub mod c1ctrl;
#[doc = "C1DTCNT (rw) register accessor: DMA channel 1 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1dtcnt`]
module"]
pub type C1DTCNT = crate::Reg<c1dtcnt::C1DTCNT_SPEC>;
#[doc = "DMA channel 1 number of data to transfer register"]
pub mod c1dtcnt;
#[doc = "C1PADDR (rw) register accessor: DMA channel 1 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1paddr`]
module"]
pub type C1PADDR = crate::Reg<c1paddr::C1PADDR_SPEC>;
#[doc = "DMA channel 1 peripheral base address register"]
pub mod c1paddr;
#[doc = "C1MADDR (rw) register accessor: DMA channel 1 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1maddr`]
module"]
pub type C1MADDR = crate::Reg<c1maddr::C1MADDR_SPEC>;
#[doc = "DMA channel 1 memory base address register"]
pub mod c1maddr;
#[doc = "C2CTRL (rw) register accessor: DMA channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c2ctrl`]
module"]
pub type C2CTRL = crate::Reg<c2ctrl::C2CTRL_SPEC>;
#[doc = "DMA channel configuration register"]
pub mod c2ctrl;
#[doc = "C2DTCNT (rw) register accessor: DMA channel 2 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c2dtcnt`]
module"]
pub type C2DTCNT = crate::Reg<c2dtcnt::C2DTCNT_SPEC>;
#[doc = "DMA channel 2 number of data to transfer register"]
pub mod c2dtcnt;
#[doc = "C2PADDR (rw) register accessor: DMA channel 2 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c2paddr`]
module"]
pub type C2PADDR = crate::Reg<c2paddr::C2PADDR_SPEC>;
#[doc = "DMA channel 2 peripheral base address register"]
pub mod c2paddr;
#[doc = "C2MADDR (rw) register accessor: DMA channel 2 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c2maddr`]
module"]
pub type C2MADDR = crate::Reg<c2maddr::C2MADDR_SPEC>;
#[doc = "DMA channel 2 memory base address register"]
pub mod c2maddr;
#[doc = "C3CTRL (rw) register accessor: DMA channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c3ctrl`]
module"]
pub type C3CTRL = crate::Reg<c3ctrl::C3CTRL_SPEC>;
#[doc = "DMA channel configuration register"]
pub mod c3ctrl;
#[doc = "C3DTCNT (rw) register accessor: DMA channel 3 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c3dtcnt`]
module"]
pub type C3DTCNT = crate::Reg<c3dtcnt::C3DTCNT_SPEC>;
#[doc = "DMA channel 3 number of data to transfer register"]
pub mod c3dtcnt;
#[doc = "C3PADDR (rw) register accessor: DMA channel 3 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c3paddr`]
module"]
pub type C3PADDR = crate::Reg<c3paddr::C3PADDR_SPEC>;
#[doc = "DMA channel 3 peripheral base address register"]
pub mod c3paddr;
#[doc = "C3MADDR (rw) register accessor: DMA channel 3 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c3maddr`]
module"]
pub type C3MADDR = crate::Reg<c3maddr::C3MADDR_SPEC>;
#[doc = "DMA channel 3 memory base address register"]
pub mod c3maddr;
#[doc = "C4CTRL (rw) register accessor: DMA channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c4ctrl`]
module"]
pub type C4CTRL = crate::Reg<c4ctrl::C4CTRL_SPEC>;
#[doc = "DMA channel configuration register"]
pub mod c4ctrl;
#[doc = "C4DTCNT (rw) register accessor: DMA channel 4 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c4dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c4dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c4dtcnt`]
module"]
pub type C4DTCNT = crate::Reg<c4dtcnt::C4DTCNT_SPEC>;
#[doc = "DMA channel 4 number of data to transfer register"]
pub mod c4dtcnt;
#[doc = "C4PADDR (rw) register accessor: DMA channel 4 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c4paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c4paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c4paddr`]
module"]
pub type C4PADDR = crate::Reg<c4paddr::C4PADDR_SPEC>;
#[doc = "DMA channel 4 peripheral base address register"]
pub mod c4paddr;
#[doc = "C4MADDR (rw) register accessor: DMA channel 4 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c4maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c4maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c4maddr`]
module"]
pub type C4MADDR = crate::Reg<c4maddr::C4MADDR_SPEC>;
#[doc = "DMA channel 4 memory base address register"]
pub mod c4maddr;
#[doc = "C5CTRL (rw) register accessor: DMA channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c5ctrl`]
module"]
pub type C5CTRL = crate::Reg<c5ctrl::C5CTRL_SPEC>;
#[doc = "DMA channel configuration register"]
pub mod c5ctrl;
#[doc = "C5DTCNT (rw) register accessor: DMA channel 5 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c5dtcnt`]
module"]
pub type C5DTCNT = crate::Reg<c5dtcnt::C5DTCNT_SPEC>;
#[doc = "DMA channel 5 number of data to transfer register"]
pub mod c5dtcnt;
#[doc = "C5PADDR (rw) register accessor: DMA channel 5 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c5paddr`]
module"]
pub type C5PADDR = crate::Reg<c5paddr::C5PADDR_SPEC>;
#[doc = "DMA channel 5 peripheral base address register"]
pub mod c5paddr;
#[doc = "C5MADDR (rw) register accessor: DMA channel 5 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c5maddr`]
module"]
pub type C5MADDR = crate::Reg<c5maddr::C5MADDR_SPEC>;
#[doc = "DMA channel 5 memory base address register"]
pub mod c5maddr;
