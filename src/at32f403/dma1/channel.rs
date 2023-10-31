#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct CHANNEL {
    #[doc = "0x00 - DMA channel configuration register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - DMA channel number of data to transfer register"]
    pub dtcnt: DTCNT,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - DMA channel peripheral base address register"]
    pub paddr: PADDR,
    #[doc = "0x0c - DMA channel memory base address register"]
    pub maddr: MADDR,
    _reserved_end: [u8; 0x04],
}
#[doc = "CTRL (rw) register accessor: DMA channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DMA channel configuration register"]
pub mod ctrl;
#[doc = "DTCNT (rw) register accessor: DMA channel number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtcnt`]
module"]
pub type DTCNT = crate::Reg<dtcnt::DTCNT_SPEC>;
#[doc = "DMA channel number of data to transfer register"]
pub mod dtcnt;
#[doc = "PADDR (rw) register accessor: DMA channel peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`paddr`]
module"]
pub type PADDR = crate::Reg<paddr::PADDR_SPEC>;
#[doc = "DMA channel peripheral base address register"]
pub mod paddr;
#[doc = "MADDR (rw) register accessor: DMA channel memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maddr`]
module"]
pub type MADDR = crate::Reg<maddr::MADDR_SPEC>;
#[doc = "DMA channel memory base address register"]
pub mod maddr;
