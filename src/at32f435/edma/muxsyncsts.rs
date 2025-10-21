#[doc = "Register `MUXSYNCSTS` reader"]
pub type R = crate::R<MUXSYNCSTS_SPEC>;
#[doc = "Register `MUXSYNCSTS` writer"]
pub type W = crate::W<MUXSYNCSTS_SPEC>;
#[doc = "Field `SYNCOVF1` reader - Synchronizaton overrun interrupt flag"]
pub type SYNCOVF1_R = crate::BitReader;
#[doc = "Field `SYNCOVF2` reader - Synchronizaton overrun interrupt flag"]
pub type SYNCOVF2_R = crate::BitReader;
#[doc = "Field `SYNCOVF3` reader - Synchronizaton overrun interrupt flag"]
pub type SYNCOVF3_R = crate::BitReader;
#[doc = "Field `SYNCOVF4` reader - Synchronizaton overrun interrupt flag"]
pub type SYNCOVF4_R = crate::BitReader;
#[doc = "Field `SYNCOVF5` reader - Synchronizaton overrun interrupt flag"]
pub type SYNCOVF5_R = crate::BitReader;
#[doc = "Field `SYNCOVF6` reader - Synchronizaton overrun interrupt flag"]
pub type SYNCOVF6_R = crate::BitReader;
#[doc = "Field `SYNCOVF7` reader - Synchronizaton overrun interrupt flag"]
pub type SYNCOVF7_R = crate::BitReader;
#[doc = "Field `SYNCOVF8` reader - Synchronizaton overrun interrupt flag"]
pub type SYNCOVF8_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf1(&self) -> SYNCOVF1_R {
        SYNCOVF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf2(&self) -> SYNCOVF2_R {
        SYNCOVF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf3(&self) -> SYNCOVF3_R {
        SYNCOVF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf4(&self) -> SYNCOVF4_R {
        SYNCOVF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf5(&self) -> SYNCOVF5_R {
        SYNCOVF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf6(&self) -> SYNCOVF6_R {
        SYNCOVF6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf7(&self) -> SYNCOVF7_R {
        SYNCOVF7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf8(&self) -> SYNCOVF8_R {
        SYNCOVF8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXSYNCSTS")
            .field("syncovf1", &self.syncovf1())
            .field("syncovf2", &self.syncovf2())
            .field("syncovf3", &self.syncovf3())
            .field("syncovf4", &self.syncovf4())
            .field("syncovf5", &self.syncovf5())
            .field("syncovf6", &self.syncovf6())
            .field("syncovf7", &self.syncovf7())
            .field("syncovf8", &self.syncovf8())
            .finish()
    }
}
impl W {}
#[doc = "Channel Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxsyncsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxsyncsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXSYNCSTS_SPEC;
impl crate::RegisterSpec for MUXSYNCSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxsyncsts::R`](R) reader structure"]
impl crate::Readable for MUXSYNCSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxsyncsts::W`](W) writer structure"]
impl crate::Writable for MUXSYNCSTS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MUXSYNCSTS to value 0"]
impl crate::Resettable for MUXSYNCSTS_SPEC {}
