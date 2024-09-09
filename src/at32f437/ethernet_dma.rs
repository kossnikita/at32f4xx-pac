#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmabm: DMABM,
    dmatpd: DMATPD,
    dmarpd: DMARPD,
    dmardladdr: DMARDLADDR,
    dmatdladdr: DMATDLADDR,
    dmasts: DMASTS,
    dmaopm: DMAOPM,
    dmaie: DMAIE,
    dmamfbocnt: DMAMFBOCNT,
    _reserved9: [u8; 0x24],
    dmactd: DMACTD,
    dmacrd: DMACRD,
    dmactbaddr: DMACTBADDR,
    dmacrbaddr: DMACRBADDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Ethernet DMA bus mode register"]
    #[inline(always)]
    pub const fn dmabm(&self) -> &DMABM {
        &self.dmabm
    }
    #[doc = "0x04 - Ethernet DMA transmit poll demand register"]
    #[inline(always)]
    pub const fn dmatpd(&self) -> &DMATPD {
        &self.dmatpd
    }
    #[doc = "0x08 - EHERNET DMA receive poll demand register"]
    #[inline(always)]
    pub const fn dmarpd(&self) -> &DMARPD {
        &self.dmarpd
    }
    #[doc = "0x0c - Ethernet DMA receive descriptor list address register"]
    #[inline(always)]
    pub const fn dmardladdr(&self) -> &DMARDLADDR {
        &self.dmardladdr
    }
    #[doc = "0x10 - Ethernet DMA transmit descriptor list address register"]
    #[inline(always)]
    pub const fn dmatdladdr(&self) -> &DMATDLADDR {
        &self.dmatdladdr
    }
    #[doc = "0x14 - Ethernet DMA status register"]
    #[inline(always)]
    pub const fn dmasts(&self) -> &DMASTS {
        &self.dmasts
    }
    #[doc = "0x18 - Ethernet DMA operation mode register"]
    #[inline(always)]
    pub const fn dmaopm(&self) -> &DMAOPM {
        &self.dmaopm
    }
    #[doc = "0x1c - Ethernet DMA interrupt enable register"]
    #[inline(always)]
    pub const fn dmaie(&self) -> &DMAIE {
        &self.dmaie
    }
    #[doc = "0x20 - Ethernet DMA missed frame and buffer overflow counter register"]
    #[inline(always)]
    pub const fn dmamfbocnt(&self) -> &DMAMFBOCNT {
        &self.dmamfbocnt
    }
    #[doc = "0x48 - Ethernet DMA current host transmit descriptor register"]
    #[inline(always)]
    pub const fn dmactd(&self) -> &DMACTD {
        &self.dmactd
    }
    #[doc = "0x4c - Ethernet DMA current host receive descriptor register"]
    #[inline(always)]
    pub const fn dmacrd(&self) -> &DMACRD {
        &self.dmacrd
    }
    #[doc = "0x50 - Ethernet DMA current host transmit buffer address register"]
    #[inline(always)]
    pub const fn dmactbaddr(&self) -> &DMACTBADDR {
        &self.dmactbaddr
    }
    #[doc = "0x54 - Ethernet DMA current host receive buffer address register"]
    #[inline(always)]
    pub const fn dmacrbaddr(&self) -> &DMACRBADDR {
        &self.dmacrbaddr
    }
}
#[doc = "DMABM (rw) register accessor: Ethernet DMA bus mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmabm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmabm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmabm`]
module"]
pub type DMABM = crate::Reg<dmabm::DMABM_SPEC>;
#[doc = "Ethernet DMA bus mode register"]
pub mod dmabm;
#[doc = "DMATPD (rw) register accessor: Ethernet DMA transmit poll demand register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatpd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatpd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatpd`]
module"]
pub type DMATPD = crate::Reg<dmatpd::DMATPD_SPEC>;
#[doc = "Ethernet DMA transmit poll demand register"]
pub mod dmatpd;
#[doc = "DMARPD (rw) register accessor: EHERNET DMA receive poll demand register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarpd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarpd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarpd`]
module"]
pub type DMARPD = crate::Reg<dmarpd::DMARPD_SPEC>;
#[doc = "EHERNET DMA receive poll demand register"]
pub mod dmarpd;
#[doc = "DMARDLADDR (rw) register accessor: Ethernet DMA receive descriptor list address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmardladdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmardladdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmardladdr`]
module"]
pub type DMARDLADDR = crate::Reg<dmardladdr::DMARDLADDR_SPEC>;
#[doc = "Ethernet DMA receive descriptor list address register"]
pub mod dmardladdr;
#[doc = "DMATDLADDR (rw) register accessor: Ethernet DMA transmit descriptor list address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatdladdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatdladdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatdladdr`]
module"]
pub type DMATDLADDR = crate::Reg<dmatdladdr::DMATDLADDR_SPEC>;
#[doc = "Ethernet DMA transmit descriptor list address register"]
pub mod dmatdladdr;
#[doc = "DMASTS (rw) register accessor: Ethernet DMA status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasts`]
module"]
pub type DMASTS = crate::Reg<dmasts::DMASTS_SPEC>;
#[doc = "Ethernet DMA status register"]
pub mod dmasts;
#[doc = "DMAOPM (rw) register accessor: Ethernet DMA operation mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaopm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaopm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaopm`]
module"]
pub type DMAOPM = crate::Reg<dmaopm::DMAOPM_SPEC>;
#[doc = "Ethernet DMA operation mode register"]
pub mod dmaopm;
#[doc = "DMAIE (rw) register accessor: Ethernet DMA interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaie`]
module"]
pub type DMAIE = crate::Reg<dmaie::DMAIE_SPEC>;
#[doc = "Ethernet DMA interrupt enable register"]
pub mod dmaie;
#[doc = "DMAMFBOCNT (r) register accessor: Ethernet DMA missed frame and buffer overflow counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmamfbocnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamfbocnt`]
module"]
pub type DMAMFBOCNT = crate::Reg<dmamfbocnt::DMAMFBOCNT_SPEC>;
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub mod dmamfbocnt;
#[doc = "DMACTD (r) register accessor: Ethernet DMA current host transmit descriptor register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactd`]
module"]
pub type DMACTD = crate::Reg<dmactd::DMACTD_SPEC>;
#[doc = "Ethernet DMA current host transmit descriptor register"]
pub mod dmactd;
#[doc = "DMACRD (r) register accessor: Ethernet DMA current host receive descriptor register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacrd`]
module"]
pub type DMACRD = crate::Reg<dmacrd::DMACRD_SPEC>;
#[doc = "Ethernet DMA current host receive descriptor register"]
pub mod dmacrd;
#[doc = "DMACTBADDR (r) register accessor: Ethernet DMA current host transmit buffer address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactbaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactbaddr`]
module"]
pub type DMACTBADDR = crate::Reg<dmactbaddr::DMACTBADDR_SPEC>;
#[doc = "Ethernet DMA current host transmit buffer address register"]
pub mod dmactbaddr;
#[doc = "DMACRBADDR (r) register accessor: Ethernet DMA current host receive buffer address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrbaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacrbaddr`]
module"]
pub type DMACRBADDR = crate::Reg<dmacrbaddr::DMACRBADDR_SPEC>;
#[doc = "Ethernet DMA current host receive buffer address register"]
pub mod dmacrbaddr;
