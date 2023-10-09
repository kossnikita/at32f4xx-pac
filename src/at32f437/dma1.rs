#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register (DMA_STS)"]
    pub sts: STS,
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_CLR)"]
    pub clr: CLR,
    #[doc = "0x08..0x94 - DMA Channel %s"]
    pub channel: [CHANNEL; 7],
    _reserved3: [u8; 0x6c],
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
impl RegisterBlock {
    #[doc = "0x08..0x1c - DMA Channel %s"]
    #[inline(always)]
    pub fn channel1(&self) -> &CHANNEL {
        &self.channel[0]
    }
    #[doc = "0x1c..0x30 - DMA Channel %s"]
    #[inline(always)]
    pub fn channel2(&self) -> &CHANNEL {
        &self.channel[1]
    }
    #[doc = "0x30..0x44 - DMA Channel %s"]
    #[inline(always)]
    pub fn channel3(&self) -> &CHANNEL {
        &self.channel[2]
    }
    #[doc = "0x44..0x58 - DMA Channel %s"]
    #[inline(always)]
    pub fn channel4(&self) -> &CHANNEL {
        &self.channel[3]
    }
    #[doc = "0x58..0x6c - DMA Channel %s"]
    #[inline(always)]
    pub fn channel5(&self) -> &CHANNEL {
        &self.channel[4]
    }
    #[doc = "0x6c..0x80 - DMA Channel %s"]
    #[inline(always)]
    pub fn channel6(&self) -> &CHANNEL {
        &self.channel[5]
    }
    #[doc = "0x80..0x94 - DMA Channel %s"]
    #[inline(always)]
    pub fn channel7(&self) -> &CHANNEL {
        &self.channel[6]
    }
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
#[doc = "DMA Channel %s"]
pub use self::channel::CHANNEL;
#[doc = r"Cluster"]
#[doc = "DMA Channel %s"]
pub mod channel;
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
