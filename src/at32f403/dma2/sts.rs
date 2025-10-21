#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Field `GF1` reader - Channel 1 Global event flag"]
pub type GF1_R = crate::BitReader;
#[doc = "Field `FDTF1` reader - Channel 1 full data transfer event flag"]
pub type FDTF1_R = crate::BitReader;
#[doc = "Field `HDTF1` reader - Channel 1 half data transfer event flag"]
pub type HDTF1_R = crate::BitReader;
#[doc = "Field `DTERRF1` reader - Channel 1 data transfer error event flag"]
pub type DTERRF1_R = crate::BitReader;
#[doc = "Field `GF2` reader - Channel 2 Global event flag"]
pub type GF2_R = crate::BitReader;
#[doc = "Field `FDTF2` reader - Channel 2 full data transfer event flag"]
pub type FDTF2_R = crate::BitReader;
#[doc = "Field `HDTF2` reader - Channel 2 half data transfer event flag"]
pub type HDTF2_R = crate::BitReader;
#[doc = "Field `DTERRF2` reader - Channel 2 data transfer error event flag"]
pub type DTERRF2_R = crate::BitReader;
#[doc = "Field `GF3` reader - Channel 3 Global event flag"]
pub type GF3_R = crate::BitReader;
#[doc = "Field `FDTF3` reader - Channel 3 full data transfer event flag"]
pub type FDTF3_R = crate::BitReader;
#[doc = "Field `HDTF3` reader - Channel 3 half data transfer event flag"]
pub type HDTF3_R = crate::BitReader;
#[doc = "Field `DTERRF3` reader - Channel 3 data transfer error event flag"]
pub type DTERRF3_R = crate::BitReader;
#[doc = "Field `GF4` reader - Channel 4 Global event flag"]
pub type GF4_R = crate::BitReader;
#[doc = "Field `FDTF4` reader - Channel 4 full data transfer event flag"]
pub type FDTF4_R = crate::BitReader;
#[doc = "Field `HDTF4` reader - Channel 4 half data transfer event flag"]
pub type HDTF4_R = crate::BitReader;
#[doc = "Field `DTERRF4` reader - Channel 4 data transfer error event flag"]
pub type DTERRF4_R = crate::BitReader;
#[doc = "Field `GF5` reader - Channel 5 Global event flag"]
pub type GF5_R = crate::BitReader;
#[doc = "Field `FDTF5` reader - Channel 5 full data transfer event flag"]
pub type FDTF5_R = crate::BitReader;
#[doc = "Field `HDTF5` reader - Channel 5 half data transfer event flag"]
pub type HDTF5_R = crate::BitReader;
#[doc = "Field `DTERRF5` reader - Channel 5 data transfer error event flag"]
pub type DTERRF5_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 1 Global event flag"]
    #[inline(always)]
    pub fn gf1(&self) -> GF1_R {
        GF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 full data transfer event flag"]
    #[inline(always)]
    pub fn fdtf1(&self) -> FDTF1_R {
        FDTF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 half data transfer event flag"]
    #[inline(always)]
    pub fn hdtf1(&self) -> HDTF1_R {
        HDTF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf1(&self) -> DTERRF1_R {
        DTERRF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 Global event flag"]
    #[inline(always)]
    pub fn gf2(&self) -> GF2_R {
        GF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 full data transfer event flag"]
    #[inline(always)]
    pub fn fdtf2(&self) -> FDTF2_R {
        FDTF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 2 half data transfer event flag"]
    #[inline(always)]
    pub fn hdtf2(&self) -> HDTF2_R {
        HDTF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 2 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf2(&self) -> DTERRF2_R {
        DTERRF2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 3 Global event flag"]
    #[inline(always)]
    pub fn gf3(&self) -> GF3_R {
        GF3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 3 full data transfer event flag"]
    #[inline(always)]
    pub fn fdtf3(&self) -> FDTF3_R {
        FDTF3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 3 half data transfer event flag"]
    #[inline(always)]
    pub fn hdtf3(&self) -> HDTF3_R {
        HDTF3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf3(&self) -> DTERRF3_R {
        DTERRF3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Global event flag"]
    #[inline(always)]
    pub fn gf4(&self) -> GF4_R {
        GF4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 4 full data transfer event flag"]
    #[inline(always)]
    pub fn fdtf4(&self) -> FDTF4_R {
        FDTF4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 4 half data transfer event flag"]
    #[inline(always)]
    pub fn hdtf4(&self) -> HDTF4_R {
        HDTF4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 4 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf4(&self) -> DTERRF4_R {
        DTERRF4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 5 Global event flag"]
    #[inline(always)]
    pub fn gf5(&self) -> GF5_R {
        GF5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 5 full data transfer event flag"]
    #[inline(always)]
    pub fn fdtf5(&self) -> FDTF5_R {
        FDTF5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 5 half data transfer event flag"]
    #[inline(always)]
    pub fn hdtf5(&self) -> HDTF5_R {
        HDTF5_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 5 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf5(&self) -> DTERRF5_R {
        DTERRF5_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("gf1", &self.gf1())
            .field("gf2", &self.gf2())
            .field("gf3", &self.gf3())
            .field("gf4", &self.gf4())
            .field("gf5", &self.gf5())
            .field("fdtf1", &self.fdtf1())
            .field("fdtf2", &self.fdtf2())
            .field("fdtf3", &self.fdtf3())
            .field("fdtf4", &self.fdtf4())
            .field("fdtf5", &self.fdtf5())
            .field("hdtf1", &self.hdtf1())
            .field("hdtf2", &self.hdtf2())
            .field("hdtf3", &self.hdtf3())
            .field("hdtf4", &self.hdtf4())
            .field("hdtf5", &self.hdtf5())
            .field("dterrf1", &self.dterrf1())
            .field("dterrf2", &self.dterrf2())
            .field("dterrf3", &self.dterrf3())
            .field("dterrf4", &self.dterrf4())
            .field("dterrf5", &self.dterrf5())
            .finish()
    }
}
#[doc = "DMA interrupt status register (DMA_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {}
