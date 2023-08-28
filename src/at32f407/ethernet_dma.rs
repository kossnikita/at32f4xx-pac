#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet DMA bus mode register"]
    pub dmabm: DMABM,
    #[doc = "0x04 - Ethernet DMA transmit poll demand register"]
    pub dmatpd: DMATPD,
    #[doc = "0x08 - EHERNET DMA receive poll demand register"]
    pub dmarpd: DMARPD,
    #[doc = "0x0c - Ethernet DMA receive descriptor list address register"]
    pub dmardladdr: DMARDLADDR,
    #[doc = "0x10 - Ethernet DMA transmit descriptor list address register"]
    pub dmatdladdr: DMATDLADDR,
    #[doc = "0x14 - Ethernet DMA status register"]
    pub dmasts: DMASTS,
    #[doc = "0x18 - Ethernet DMA operation mode register"]
    pub dmaopm: DMAOPM,
    #[doc = "0x1c - Ethernet DMA interrupt enable register"]
    pub dmaie: DMAIE,
    #[doc = "0x20 - Ethernet DMA missed frame and buffer overflow counter register"]
    pub dmamfbocnt: DMAMFBOCNT,
    _reserved9: [u8; 0x24],
    #[doc = "0x48 - Ethernet DMA current host transmit descriptor register"]
    pub dmactd: DMACTD,
    #[doc = "0x4c - Ethernet DMA current host receive descriptor register"]
    pub dmacrd: DMACRD,
    #[doc = "0x50 - Ethernet DMA current host transmit buffer address register"]
    pub dmactbaddr: DMACTBADDR,
    #[doc = "0x54 - Ethernet DMA current host receive buffer address register"]
    pub dmacrbaddr: DMACRBADDR,
}
#[doc = "DMABM (rw) register accessor: Ethernet DMA bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmabm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmabm`]
module"]
pub type DMABM = crate::Reg<dmabm::DMABM_SPEC>;
#[doc = "Ethernet DMA bus mode register"]
pub mod dmabm;
#[doc = "DMATPD (rw) register accessor: Ethernet DMA transmit poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatpd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatpd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmatpd`]
module"]
pub type DMATPD = crate::Reg<dmatpd::DMATPD_SPEC>;
#[doc = "Ethernet DMA transmit poll demand register"]
pub mod dmatpd;
#[doc = "DMARPD (rw) register accessor: EHERNET DMA receive poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarpd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarpd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmarpd`]
module"]
pub type DMARPD = crate::Reg<dmarpd::DMARPD_SPEC>;
#[doc = "EHERNET DMA receive poll demand register"]
pub mod dmarpd;
#[doc = "DMARDLADDR (rw) register accessor: Ethernet DMA receive descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmardladdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmardladdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmardladdr`]
module"]
pub type DMARDLADDR = crate::Reg<dmardladdr::DMARDLADDR_SPEC>;
#[doc = "Ethernet DMA receive descriptor list address register"]
pub mod dmardladdr;
#[doc = "DMATDLADDR (rw) register accessor: Ethernet DMA transmit descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatdladdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatdladdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmatdladdr`]
module"]
pub type DMATDLADDR = crate::Reg<dmatdladdr::DMATDLADDR_SPEC>;
#[doc = "Ethernet DMA transmit descriptor list address register"]
pub mod dmatdladdr;
#[doc = "DMASTS (rw) register accessor: Ethernet DMA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmasts`]
module"]
pub type DMASTS = crate::Reg<dmasts::DMASTS_SPEC>;
#[doc = "Ethernet DMA status register"]
pub mod dmasts;
#[doc = "DMAOPM (rw) register accessor: Ethernet DMA operation mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaopm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaopm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaopm`]
module"]
pub type DMAOPM = crate::Reg<dmaopm::DMAOPM_SPEC>;
#[doc = "Ethernet DMA operation mode register"]
pub mod dmaopm;
#[doc = "DMAIE (rw) register accessor: Ethernet DMA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmaie`]
module"]
pub type DMAIE = crate::Reg<dmaie::DMAIE_SPEC>;
#[doc = "Ethernet DMA interrupt enable register"]
pub mod dmaie;
#[doc = "DMAMFBOCNT (r) register accessor: Ethernet DMA missed frame and buffer overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamfbocnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmamfbocnt`]
module"]
pub type DMAMFBOCNT = crate::Reg<dmamfbocnt::DMAMFBOCNT_SPEC>;
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub mod dmamfbocnt;
#[doc = "DMACTD (r) register accessor: Ethernet DMA current host transmit descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmactd`]
module"]
pub type DMACTD = crate::Reg<dmactd::DMACTD_SPEC>;
#[doc = "Ethernet DMA current host transmit descriptor register"]
pub mod dmactd;
#[doc = "DMACRD (r) register accessor: Ethernet DMA current host receive descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmacrd`]
module"]
pub type DMACRD = crate::Reg<dmacrd::DMACRD_SPEC>;
#[doc = "Ethernet DMA current host receive descriptor register"]
pub mod dmacrd;
#[doc = "DMACTBADDR (r) register accessor: Ethernet DMA current host transmit buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactbaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmactbaddr`]
module"]
pub type DMACTBADDR = crate::Reg<dmactbaddr::DMACTBADDR_SPEC>;
#[doc = "Ethernet DMA current host transmit buffer address register"]
pub mod dmactbaddr;
#[doc = "DMACRBADDR (r) register accessor: Ethernet DMA current host receive buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrbaddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmacrbaddr`]
module"]
pub type DMACRBADDR = crate::Reg<dmacrbaddr::DMACRBADDR_SPEC>;
#[doc = "Ethernet DMA current host receive buffer address register"]
pub mod dmacrbaddr;
