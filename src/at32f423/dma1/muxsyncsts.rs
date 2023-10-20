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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXSYNCSTS")
            .field("syncovf1", &format_args!("{}", self.syncovf1().bit()))
            .field("syncovf2", &format_args!("{}", self.syncovf2().bit()))
            .field("syncovf3", &format_args!("{}", self.syncovf3().bit()))
            .field("syncovf4", &format_args!("{}", self.syncovf4().bit()))
            .field("syncovf5", &format_args!("{}", self.syncovf5().bit()))
            .field("syncovf6", &format_args!("{}", self.syncovf6().bit()))
            .field("syncovf7", &format_args!("{}", self.syncovf7().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MUXSYNCSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxsyncsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxsyncsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXSYNCSTS_SPEC;
impl crate::RegisterSpec for MUXSYNCSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxsyncsts::R`](R) reader structure"]
impl crate::Readable for MUXSYNCSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxsyncsts::W`](W) writer structure"]
impl crate::Writable for MUXSYNCSTS_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXSYNCSTS to value 0"]
impl crate::Resettable for MUXSYNCSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
