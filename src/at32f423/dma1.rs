#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sts: STS,
    clr: CLR,
    channel: [Channel; 7],
    _reserved3: [u8; 0x6c],
    dma_muxsel: DMA_MUXSEL,
    muxc1ctrl: MUXC1CTRL,
    muxc2ctrl: MUXC2CTRL,
    muxc3ctrl: MUXC3CTRL,
    muxc4ctrl: MUXC4CTRL,
    muxc5ctrl: MUXC5CTRL,
    muxc6ctrl: MUXC6CTRL,
    muxc7ctrl: MUXC7CTRL,
    muxg1ctrl: MUXG1CTRL,
    muxg2ctrl: MUXG2CTRL,
    muxg3ctrl: MUXG3CTRL,
    muxg4ctrl: MUXG4CTRL,
    muxsyncsts: MUXSYNCSTS,
    muxsyncclr: MUXSYNCCLR,
    muxgsts: MUXGSTS,
    muxgclr: MUXGCLR,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register (DMA_STS)"]
    #[inline(always)]
    pub const fn sts(&self) -> &STS {
        &self.sts
    }
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_CLR)"]
    #[inline(always)]
    pub const fn clr(&self) -> &CLR {
        &self.clr
    }
    #[doc = "0x08..0x94 - DMA Channel %s"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        &self.channel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x94 - DMA Channel %s"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        self.channel.iter()
    }
    #[doc = "0x08..0x1c - DMA Channel 1"]
    #[inline(always)]
    pub const fn channel1(&self) -> &Channel {
        self.channel(0)
    }
    #[doc = "0x1c..0x30 - DMA Channel 2"]
    #[inline(always)]
    pub const fn channel2(&self) -> &Channel {
        self.channel(1)
    }
    #[doc = "0x30..0x44 - DMA Channel 3"]
    #[inline(always)]
    pub const fn channel3(&self) -> &Channel {
        self.channel(2)
    }
    #[doc = "0x44..0x58 - DMA Channel 4"]
    #[inline(always)]
    pub const fn channel4(&self) -> &Channel {
        self.channel(3)
    }
    #[doc = "0x58..0x6c - DMA Channel 5"]
    #[inline(always)]
    pub const fn channel5(&self) -> &Channel {
        self.channel(4)
    }
    #[doc = "0x6c..0x80 - DMA Channel 6"]
    #[inline(always)]
    pub const fn channel6(&self) -> &Channel {
        self.channel(5)
    }
    #[doc = "0x80..0x94 - DMA Channel 7"]
    #[inline(always)]
    pub const fn channel7(&self) -> &Channel {
        self.channel(6)
    }
    #[doc = "0x100 - DMAMUX Table Selection"]
    #[inline(always)]
    pub const fn dma_muxsel(&self) -> &DMA_MUXSEL {
        &self.dma_muxsel
    }
    #[doc = "0x104 - Channel 1 Configuration Register"]
    #[inline(always)]
    pub const fn muxc1ctrl(&self) -> &MUXC1CTRL {
        &self.muxc1ctrl
    }
    #[doc = "0x108 - Channel 2 Configuration Register"]
    #[inline(always)]
    pub const fn muxc2ctrl(&self) -> &MUXC2CTRL {
        &self.muxc2ctrl
    }
    #[doc = "0x10c - Channel 3 Configuration Register"]
    #[inline(always)]
    pub const fn muxc3ctrl(&self) -> &MUXC3CTRL {
        &self.muxc3ctrl
    }
    #[doc = "0x110 - Channel 4 Configuration Register"]
    #[inline(always)]
    pub const fn muxc4ctrl(&self) -> &MUXC4CTRL {
        &self.muxc4ctrl
    }
    #[doc = "0x114 - Channel 5 Configuration Register"]
    #[inline(always)]
    pub const fn muxc5ctrl(&self) -> &MUXC5CTRL {
        &self.muxc5ctrl
    }
    #[doc = "0x118 - Channel 6 Configuration Register"]
    #[inline(always)]
    pub const fn muxc6ctrl(&self) -> &MUXC6CTRL {
        &self.muxc6ctrl
    }
    #[doc = "0x11c - Channel 7 Configuration Register"]
    #[inline(always)]
    pub const fn muxc7ctrl(&self) -> &MUXC7CTRL {
        &self.muxc7ctrl
    }
    #[doc = "0x120 - Generator 1 Configuration Register"]
    #[inline(always)]
    pub const fn muxg1ctrl(&self) -> &MUXG1CTRL {
        &self.muxg1ctrl
    }
    #[doc = "0x124 - Generator 2 Configuration Register"]
    #[inline(always)]
    pub const fn muxg2ctrl(&self) -> &MUXG2CTRL {
        &self.muxg2ctrl
    }
    #[doc = "0x128 - Generator 3 Configuration Register"]
    #[inline(always)]
    pub const fn muxg3ctrl(&self) -> &MUXG3CTRL {
        &self.muxg3ctrl
    }
    #[doc = "0x12c - Generator 4 Configuration Register"]
    #[inline(always)]
    pub const fn muxg4ctrl(&self) -> &MUXG4CTRL {
        &self.muxg4ctrl
    }
    #[doc = "0x130 - Channel Interrupt Status Register"]
    #[inline(always)]
    pub const fn muxsyncsts(&self) -> &MUXSYNCSTS {
        &self.muxsyncsts
    }
    #[doc = "0x134 - Channel Interrupt Clear Flag Register"]
    #[inline(always)]
    pub const fn muxsyncclr(&self) -> &MUXSYNCCLR {
        &self.muxsyncclr
    }
    #[doc = "0x138 - Generator Interrupt Status Register"]
    #[inline(always)]
    pub const fn muxgsts(&self) -> &MUXGSTS {
        &self.muxgsts
    }
    #[doc = "0x13c - Generator Interrupt Clear Flag Register"]
    #[inline(always)]
    pub const fn muxgclr(&self) -> &MUXGCLR {
        &self.muxgclr
    }
}
#[doc = "STS (r) register accessor: DMA interrupt status register (DMA_STS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "DMA interrupt status register (DMA_STS)"]
pub mod sts;
#[doc = "CLR (rw) register accessor: DMA interrupt flag clear register (DMA_CLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "DMA interrupt flag clear register (DMA_CLR)"]
pub mod clr;
#[doc = "DMA Channel %s"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "DMA Channel %s"]
pub mod channel;
#[doc = "DMA_MUXSEL (rw) register accessor: DMAMUX Table Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_muxsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_muxsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_muxsel`]
module"]
pub type DMA_MUXSEL = crate::Reg<dma_muxsel::DMA_MUXSEL_SPEC>;
#[doc = "DMAMUX Table Selection"]
pub mod dma_muxsel;
#[doc = "MUXC1CTRL (rw) register accessor: Channel 1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxc1ctrl`]
module"]
pub type MUXC1CTRL = crate::Reg<muxc1ctrl::MUXC1CTRL_SPEC>;
#[doc = "Channel 1 Configuration Register"]
pub mod muxc1ctrl;
#[doc = "MUXC2CTRL (rw) register accessor: Channel 2 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxc2ctrl`]
module"]
pub type MUXC2CTRL = crate::Reg<muxc2ctrl::MUXC2CTRL_SPEC>;
#[doc = "Channel 2 Configuration Register"]
pub mod muxc2ctrl;
#[doc = "MUXC3CTRL (rw) register accessor: Channel 3 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxc3ctrl`]
module"]
pub type MUXC3CTRL = crate::Reg<muxc3ctrl::MUXC3CTRL_SPEC>;
#[doc = "Channel 3 Configuration Register"]
pub mod muxc3ctrl;
#[doc = "MUXC4CTRL (rw) register accessor: Channel 4 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxc4ctrl`]
module"]
pub type MUXC4CTRL = crate::Reg<muxc4ctrl::MUXC4CTRL_SPEC>;
#[doc = "Channel 4 Configuration Register"]
pub mod muxc4ctrl;
#[doc = "MUXC5CTRL (rw) register accessor: Channel 5 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc5ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc5ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxc5ctrl`]
module"]
pub type MUXC5CTRL = crate::Reg<muxc5ctrl::MUXC5CTRL_SPEC>;
#[doc = "Channel 5 Configuration Register"]
pub mod muxc5ctrl;
#[doc = "MUXC6CTRL (rw) register accessor: Channel 6 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc6ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc6ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxc6ctrl`]
module"]
pub type MUXC6CTRL = crate::Reg<muxc6ctrl::MUXC6CTRL_SPEC>;
#[doc = "Channel 6 Configuration Register"]
pub mod muxc6ctrl;
#[doc = "MUXC7CTRL (rw) register accessor: Channel 7 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc7ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc7ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxc7ctrl`]
module"]
pub type MUXC7CTRL = crate::Reg<muxc7ctrl::MUXC7CTRL_SPEC>;
#[doc = "Channel 7 Configuration Register"]
pub mod muxc7ctrl;
#[doc = "MUXG1CTRL (rw) register accessor: Generator 1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxg1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxg1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxg1ctrl`]
module"]
pub type MUXG1CTRL = crate::Reg<muxg1ctrl::MUXG1CTRL_SPEC>;
#[doc = "Generator 1 Configuration Register"]
pub mod muxg1ctrl;
#[doc = "MUXG2CTRL (rw) register accessor: Generator 2 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxg2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxg2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxg2ctrl`]
module"]
pub type MUXG2CTRL = crate::Reg<muxg2ctrl::MUXG2CTRL_SPEC>;
#[doc = "Generator 2 Configuration Register"]
pub mod muxg2ctrl;
#[doc = "MUXG3CTRL (rw) register accessor: Generator 3 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxg3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxg3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxg3ctrl`]
module"]
pub type MUXG3CTRL = crate::Reg<muxg3ctrl::MUXG3CTRL_SPEC>;
#[doc = "Generator 3 Configuration Register"]
pub mod muxg3ctrl;
#[doc = "MUXG4CTRL (rw) register accessor: Generator 4 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxg4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxg4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxg4ctrl`]
module"]
pub type MUXG4CTRL = crate::Reg<muxg4ctrl::MUXG4CTRL_SPEC>;
#[doc = "Generator 4 Configuration Register"]
pub mod muxg4ctrl;
#[doc = "MUXSYNCSTS (rw) register accessor: Channel Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxsyncsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxsyncsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxsyncsts`]
module"]
pub type MUXSYNCSTS = crate::Reg<muxsyncsts::MUXSYNCSTS_SPEC>;
#[doc = "Channel Interrupt Status Register"]
pub mod muxsyncsts;
#[doc = "MUXSYNCCLR (rw) register accessor: Channel Interrupt Clear Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxsyncclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxsyncclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxsyncclr`]
module"]
pub type MUXSYNCCLR = crate::Reg<muxsyncclr::MUXSYNCCLR_SPEC>;
#[doc = "Channel Interrupt Clear Flag Register"]
pub mod muxsyncclr;
#[doc = "MUXGSTS (rw) register accessor: Generator Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxgsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxgsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxgsts`]
module"]
pub type MUXGSTS = crate::Reg<muxgsts::MUXGSTS_SPEC>;
#[doc = "Generator Interrupt Status Register"]
pub mod muxgsts;
#[doc = "MUXGCLR (rw) register accessor: Generator Interrupt Clear Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxgclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxgclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxgclr`]
module"]
pub type MUXGCLR = crate::Reg<muxgclr::MUXGCLR_SPEC>;
#[doc = "Generator Interrupt Clear Flag Register"]
pub mod muxgclr;
