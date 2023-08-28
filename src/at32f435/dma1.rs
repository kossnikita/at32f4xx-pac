#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register (DMA_STS)"]
    pub sts: STS,
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_CLR)"]
    pub clr: CLR,
    #[doc = "0x08 - DMA channel configuration register(DMA_C1CTRL)"]
    pub c1ctrl: C1CTRL,
    #[doc = "0x0c - DMA channel 1 number of data to transfer register"]
    pub c1dtcnt: C1DTCNT,
    #[doc = "0x10 - DMA channel 1 peripheral base address register"]
    pub c1paddr: C1PADDR,
    #[doc = "0x14 - DMA channel 1 memory base address register"]
    pub c1maddr: C1MADDR,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - DMA channel configuration register (DMA_C2CTRL)"]
    pub c2ctrl: C2CTRL,
    #[doc = "0x20 - DMA channel 2 number of data to transferregister"]
    pub c2dtcnt: C2DTCNT,
    #[doc = "0x24 - DMA channel 2 peripheral base address register"]
    pub c2paddr: C2PADDR,
    #[doc = "0x28 - DMA channel 2 memory base address register"]
    pub c2maddr: C2MADDR,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - DMA channel configuration register (DMA_C3CTRL)"]
    pub c3ctrl: C3CTRL,
    #[doc = "0x34 - DMA channel 3 number of data to transfer register"]
    pub c3dtcnt: C3DTCNT,
    #[doc = "0x38 - DMA channel 3 peripheral base address register"]
    pub c3paddr: C3PADDR,
    #[doc = "0x3c - DMA channel 3 memory base address register"]
    pub c3maddr: C3MADDR,
    _reserved14: [u8; 0x04],
    #[doc = "0x44 - DMA channel configuration register (DMA_C4CTRL)"]
    pub c4ctrl: C4CTRL,
    #[doc = "0x48 - DMA channel 4 number of data to transfer register"]
    pub c4dtcnt: C4DTCNT,
    #[doc = "0x4c - DMA channel 4 peripheral base address register"]
    pub c4paddr: C4PADDR,
    #[doc = "0x50 - DMA channel 4 memory base address register"]
    pub c4maddr: C4MADDR,
    _reserved18: [u8; 0x04],
    #[doc = "0x58 - DMA channel configuration register (DMA_C5CTRL)"]
    pub c5ctrl: C5CTRL,
    #[doc = "0x5c - DMA channel 5 number of data to transfer register"]
    pub c5dtcnt: C5DTCNT,
    #[doc = "0x60 - DMA channel 5 peripheral base address register"]
    pub c5paddr: C5PADDR,
    #[doc = "0x64 - DMA channel 5 memory base address register"]
    pub c5maddr: C5MADDR,
    _reserved22: [u8; 0x04],
    #[doc = "0x6c - DMA channel configuration register(DMA_C6CTRL)"]
    pub c6ctrl: C6CTRL,
    #[doc = "0x70 - DMA channel 6 number of data to transfer register"]
    pub c6dtcnt: C6DTCNT,
    #[doc = "0x74 - DMA channel 6 peripheral address base register"]
    pub c6paddr: C6PADDR,
    #[doc = "0x78 - DMA channel 6 memory address base register"]
    pub c6maddr: C6MADDR,
    _reserved26: [u8; 0x04],
    #[doc = "0x80 - DMA channel configuration register(DMA_C7CTRL)"]
    pub c7ctrl: C7CTRL,
    #[doc = "0x84 - DMA channel 7 number of data to transfer register"]
    pub c7dtcnt: C7DTCNT,
    #[doc = "0x88 - DMA channel 7 peripheral base address register"]
    pub c7paddr: C7PADDR,
    #[doc = "0x8c - DMA channel 7 memory base address register"]
    pub c7maddr: C7MADDR,
    _reserved30: [u8; 0x70],
    #[doc = "0x100 - DMAMUX Table Selection"]
    pub dma_muxsel: DMA_MUXSEL,
    #[doc = "0x104 - Channel 1 Configuration Register"]
    pub muxc1ctrl: MUXC1CTRL,
    #[doc = "0x108 - Channel 2 Configuration Register"]
    pub muxc2ctrl: MUXC2CTRL,
    #[doc = "0x10c - Channel 3 Configuration Register"]
    pub muxc3ctrl: MUXC3CTRL,
    #[doc = "0x110 - Channel 4 Configuration Register"]
    pub muxc4ctrl: MUXC4CTRL,
    #[doc = "0x114 - Channel 5 Configuration Register"]
    pub muxc5ctrl: MUXC5CTRL,
    #[doc = "0x118 - Channel 6 Configuration Register"]
    pub muxc6ctrl: MUXC6CTRL,
    #[doc = "0x11c - Channel 7 Configuration Register"]
    pub muxc7ctrl: MUXC7CTRL,
    #[doc = "0x120 - Generator 1 Configuration Register"]
    pub muxg1ctrl: MUXG1CTRL,
    #[doc = "0x124 - Generator 2 Configuration Register"]
    pub muxg2ctrl: MUXG2CTRL,
    #[doc = "0x128 - Generator 3 Configuration Register"]
    pub muxg3ctrl: MUXG3CTRL,
    #[doc = "0x12c - Generator 4 Configuration Register"]
    pub muxg4ctrl: MUXG4CTRL,
    #[doc = "0x130 - Channel Interrupt Status Register"]
    pub muxsyncsts: MUXSYNCSTS,
    #[doc = "0x134 - Channel Interrupt Clear Flag Register"]
    pub muxsyncclr: MUXSYNCCLR,
    #[doc = "0x138 - Generator Interrupt Status Register"]
    pub muxgsts: MUXGSTS,
    #[doc = "0x13c - Generator Interrupt Clear Flag Register"]
    pub muxgclr: MUXGCLR,
}
#[doc = "STS (r) register accessor: DMA interrupt status register (DMA_STS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "DMA interrupt status register (DMA_STS)"]
pub mod sts;
#[doc = "CLR (rw) register accessor: DMA interrupt flag clear register (DMA_CLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clr`]
module"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "DMA interrupt flag clear register (DMA_CLR)"]
pub mod clr;
#[doc = "C1CTRL (rw) register accessor: DMA channel configuration register(DMA_C1CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c1ctrl`]
module"]
pub type C1CTRL = crate::Reg<c1ctrl::C1CTRL_SPEC>;
#[doc = "DMA channel configuration register(DMA_C1CTRL)"]
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
#[doc = "C2CTRL (rw) register accessor: DMA channel configuration register (DMA_C2CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c2ctrl`]
module"]
pub type C2CTRL = crate::Reg<c2ctrl::C2CTRL_SPEC>;
#[doc = "DMA channel configuration register (DMA_C2CTRL)"]
pub mod c2ctrl;
#[doc = "C2DTCNT (rw) register accessor: DMA channel 2 number of data to transferregister\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c2dtcnt`]
module"]
pub type C2DTCNT = crate::Reg<c2dtcnt::C2DTCNT_SPEC>;
#[doc = "DMA channel 2 number of data to transferregister"]
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
#[doc = "C3CTRL (rw) register accessor: DMA channel configuration register (DMA_C3CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c3ctrl`]
module"]
pub type C3CTRL = crate::Reg<c3ctrl::C3CTRL_SPEC>;
#[doc = "DMA channel configuration register (DMA_C3CTRL)"]
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
#[doc = "C4CTRL (rw) register accessor: DMA channel configuration register (DMA_C4CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c4ctrl`]
module"]
pub type C4CTRL = crate::Reg<c4ctrl::C4CTRL_SPEC>;
#[doc = "DMA channel configuration register (DMA_C4CTRL)"]
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
#[doc = "C5CTRL (rw) register accessor: DMA channel configuration register (DMA_C5CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c5ctrl`]
module"]
pub type C5CTRL = crate::Reg<c5ctrl::C5CTRL_SPEC>;
#[doc = "DMA channel configuration register (DMA_C5CTRL)"]
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
#[doc = "C6CTRL (rw) register accessor: DMA channel configuration register(DMA_C6CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c6ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c6ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c6ctrl`]
module"]
pub type C6CTRL = crate::Reg<c6ctrl::C6CTRL_SPEC>;
#[doc = "DMA channel configuration register(DMA_C6CTRL)"]
pub mod c6ctrl;
#[doc = "C6DTCNT (rw) register accessor: DMA channel 6 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c6dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c6dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c6dtcnt`]
module"]
pub type C6DTCNT = crate::Reg<c6dtcnt::C6DTCNT_SPEC>;
#[doc = "DMA channel 6 number of data to transfer register"]
pub mod c6dtcnt;
#[doc = "C6PADDR (rw) register accessor: DMA channel 6 peripheral address base register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c6paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c6paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c6paddr`]
module"]
pub type C6PADDR = crate::Reg<c6paddr::C6PADDR_SPEC>;
#[doc = "DMA channel 6 peripheral address base register"]
pub mod c6paddr;
#[doc = "C6MADDR (rw) register accessor: DMA channel 6 memory address base register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c6maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c6maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c6maddr`]
module"]
pub type C6MADDR = crate::Reg<c6maddr::C6MADDR_SPEC>;
#[doc = "DMA channel 6 memory address base register"]
pub mod c6maddr;
#[doc = "C7CTRL (rw) register accessor: DMA channel configuration register(DMA_C7CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c7ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c7ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c7ctrl`]
module"]
pub type C7CTRL = crate::Reg<c7ctrl::C7CTRL_SPEC>;
#[doc = "DMA channel configuration register(DMA_C7CTRL)"]
pub mod c7ctrl;
#[doc = "C7DTCNT (rw) register accessor: DMA channel 7 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c7dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c7dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c7dtcnt`]
module"]
pub type C7DTCNT = crate::Reg<c7dtcnt::C7DTCNT_SPEC>;
#[doc = "DMA channel 7 number of data to transfer register"]
pub mod c7dtcnt;
#[doc = "C7PADDR (rw) register accessor: DMA channel 7 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c7paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c7paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c7paddr`]
module"]
pub type C7PADDR = crate::Reg<c7paddr::C7PADDR_SPEC>;
#[doc = "DMA channel 7 peripheral base address register"]
pub mod c7paddr;
#[doc = "C7MADDR (rw) register accessor: DMA channel 7 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c7maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c7maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`c7maddr`]
module"]
pub type C7MADDR = crate::Reg<c7maddr::C7MADDR_SPEC>;
#[doc = "DMA channel 7 memory base address register"]
pub mod c7maddr;
#[doc = "DMA_MUXSEL (rw) register accessor: DMAMUX Table Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_muxsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_muxsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma_muxsel`]
module"]
pub type DMA_MUXSEL = crate::Reg<dma_muxsel::DMA_MUXSEL_SPEC>;
#[doc = "DMAMUX Table Selection"]
pub mod dma_muxsel;
#[doc = "MUXC1CTRL (rw) register accessor: Channel 1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxc1ctrl`]
module"]
pub type MUXC1CTRL = crate::Reg<muxc1ctrl::MUXC1CTRL_SPEC>;
#[doc = "Channel 1 Configuration Register"]
pub mod muxc1ctrl;
#[doc = "MUXC2CTRL (rw) register accessor: Channel 2 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxc2ctrl`]
module"]
pub type MUXC2CTRL = crate::Reg<muxc2ctrl::MUXC2CTRL_SPEC>;
#[doc = "Channel 2 Configuration Register"]
pub mod muxc2ctrl;
#[doc = "MUXC3CTRL (rw) register accessor: Channel 3 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxc3ctrl`]
module"]
pub type MUXC3CTRL = crate::Reg<muxc3ctrl::MUXC3CTRL_SPEC>;
#[doc = "Channel 3 Configuration Register"]
pub mod muxc3ctrl;
#[doc = "MUXC4CTRL (rw) register accessor: Channel 4 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxc4ctrl`]
module"]
pub type MUXC4CTRL = crate::Reg<muxc4ctrl::MUXC4CTRL_SPEC>;
#[doc = "Channel 4 Configuration Register"]
pub mod muxc4ctrl;
#[doc = "MUXC5CTRL (rw) register accessor: Channel 5 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc5ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc5ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxc5ctrl`]
module"]
pub type MUXC5CTRL = crate::Reg<muxc5ctrl::MUXC5CTRL_SPEC>;
#[doc = "Channel 5 Configuration Register"]
pub mod muxc5ctrl;
#[doc = "MUXC6CTRL (rw) register accessor: Channel 6 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc6ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc6ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxc6ctrl`]
module"]
pub type MUXC6CTRL = crate::Reg<muxc6ctrl::MUXC6CTRL_SPEC>;
#[doc = "Channel 6 Configuration Register"]
pub mod muxc6ctrl;
#[doc = "MUXC7CTRL (rw) register accessor: Channel 7 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc7ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc7ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxc7ctrl`]
module"]
pub type MUXC7CTRL = crate::Reg<muxc7ctrl::MUXC7CTRL_SPEC>;
#[doc = "Channel 7 Configuration Register"]
pub mod muxc7ctrl;
#[doc = "MUXG1CTRL (rw) register accessor: Generator 1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxg1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxg1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxg1ctrl`]
module"]
pub type MUXG1CTRL = crate::Reg<muxg1ctrl::MUXG1CTRL_SPEC>;
#[doc = "Generator 1 Configuration Register"]
pub mod muxg1ctrl;
#[doc = "MUXG2CTRL (rw) register accessor: Generator 2 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxg2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxg2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxg2ctrl`]
module"]
pub type MUXG2CTRL = crate::Reg<muxg2ctrl::MUXG2CTRL_SPEC>;
#[doc = "Generator 2 Configuration Register"]
pub mod muxg2ctrl;
#[doc = "MUXG3CTRL (rw) register accessor: Generator 3 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxg3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxg3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxg3ctrl`]
module"]
pub type MUXG3CTRL = crate::Reg<muxg3ctrl::MUXG3CTRL_SPEC>;
#[doc = "Generator 3 Configuration Register"]
pub mod muxg3ctrl;
#[doc = "MUXG4CTRL (rw) register accessor: Generator 4 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxg4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxg4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxg4ctrl`]
module"]
pub type MUXG4CTRL = crate::Reg<muxg4ctrl::MUXG4CTRL_SPEC>;
#[doc = "Generator 4 Configuration Register"]
pub mod muxg4ctrl;
#[doc = "MUXSYNCSTS (rw) register accessor: Channel Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxsyncsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxsyncsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxsyncsts`]
module"]
pub type MUXSYNCSTS = crate::Reg<muxsyncsts::MUXSYNCSTS_SPEC>;
#[doc = "Channel Interrupt Status Register"]
pub mod muxsyncsts;
#[doc = "MUXSYNCCLR (rw) register accessor: Channel Interrupt Clear Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxsyncclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxsyncclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxsyncclr`]
module"]
pub type MUXSYNCCLR = crate::Reg<muxsyncclr::MUXSYNCCLR_SPEC>;
#[doc = "Channel Interrupt Clear Flag Register"]
pub mod muxsyncclr;
#[doc = "MUXGSTS (rw) register accessor: Generator Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxgsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxgsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxgsts`]
module"]
pub type MUXGSTS = crate::Reg<muxgsts::MUXGSTS_SPEC>;
#[doc = "Generator Interrupt Status Register"]
pub mod muxgsts;
#[doc = "MUXGCLR (rw) register accessor: Generator Interrupt Clear Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxgclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxgclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`muxgclr`]
module"]
pub type MUXGCLR = crate::Reg<muxgclr::MUXGCLR_SPEC>;
#[doc = "Generator Interrupt Clear Flag Register"]
pub mod muxgclr;
