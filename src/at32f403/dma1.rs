#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sts: STS,
    clr: CLR,
    channel: [Channel; 7],
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
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of cluster in the array. `n == 0` corresponds to `Channel1` cluster.</div>"]
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
}
#[doc = "STS (r) register accessor: DMA interrupt status register (DMA_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`] module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "DMA interrupt status register (DMA_STS)"]
pub mod sts;
#[doc = "CLR (rw) register accessor: DMA interrupt flag clear register (DMA_CLR)\n\nYou can [`read`](crate::Reg::read) this register and get [`clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`] module"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "DMA interrupt flag clear register (DMA_CLR)"]
pub mod clr;
#[doc = "DMA Channel %s"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "DMA Channel %s"]
pub mod channel;
