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
