#[doc = "Register `MUXSYNCCLR` reader"]
pub type R = crate::R<MUXSYNCCLR_SPEC>;
#[doc = "Register `MUXSYNCCLR` writer"]
pub type W = crate::W<MUXSYNCCLR_SPEC>;
#[doc = "Field `SYNCOVFC1` reader - Clear synchronizaton overrun interrupt flag"]
pub type SYNCOVFC1_R = crate::BitReader;
#[doc = "Field `SYNCOVFC1` writer - Clear synchronizaton overrun interrupt flag"]
pub type SYNCOVFC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNCOVFC2` reader - Clear synchronizaton overrun interrupt flag"]
pub type SYNCOVFC2_R = crate::BitReader;
#[doc = "Field `SYNCOVFC2` writer - Clear synchronizaton overrun interrupt flag"]
pub type SYNCOVFC2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNCOVFC3` reader - Clear synchronizaton overrun interrupt flag"]
pub type SYNCOVFC3_R = crate::BitReader;
#[doc = "Field `SYNCOVFC3` writer - Clear synchronizaton overrun interrupt flag"]
pub type SYNCOVFC3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNCOVFC4` reader - Clear synchronizaton overrun interrupt flag"]
pub type SYNCOVFC4_R = crate::BitReader;
#[doc = "Field `SYNCOVFC4` writer - Clear synchronizaton overrun interrupt flag"]
pub type SYNCOVFC4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNCOVFC5` reader - Clear synchronizaton overrun interrupt flag"]
pub type SYNCOVFC5_R = crate::BitReader;
#[doc = "Field `SYNCOVFC5` writer - Clear synchronizaton overrun interrupt flag"]
pub type SYNCOVFC5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNCOVFC6` reader - Clear synchronizaton overrun interrupt flag"]
pub type SYNCOVFC6_R = crate::BitReader;
#[doc = "Field `SYNCOVFC6` writer - Clear synchronizaton overrun interrupt flag"]
pub type SYNCOVFC6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNCOVFC7` reader - Clear synchronizaton overrun interrupt flag"]
pub type SYNCOVFC7_R = crate::BitReader;
#[doc = "Field `SYNCOVFC7` writer - Clear synchronizaton overrun interrupt flag"]
pub type SYNCOVFC7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc1(&self) -> SYNCOVFC1_R {
        SYNCOVFC1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc2(&self) -> SYNCOVFC2_R {
        SYNCOVFC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc3(&self) -> SYNCOVFC3_R {
        SYNCOVFC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc4(&self) -> SYNCOVFC4_R {
        SYNCOVFC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc5(&self) -> SYNCOVFC5_R {
        SYNCOVFC5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc6(&self) -> SYNCOVFC6_R {
        SYNCOVFC6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc7(&self) -> SYNCOVFC7_R {
        SYNCOVFC7_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXSYNCCLR")
            .field("syncovfc1", &format_args!("{}", self.syncovfc1().bit()))
            .field("syncovfc2", &format_args!("{}", self.syncovfc2().bit()))
            .field("syncovfc3", &format_args!("{}", self.syncovfc3().bit()))
            .field("syncovfc4", &format_args!("{}", self.syncovfc4().bit()))
            .field("syncovfc5", &format_args!("{}", self.syncovfc5().bit()))
            .field("syncovfc6", &format_args!("{}", self.syncovfc6().bit()))
            .field("syncovfc7", &format_args!("{}", self.syncovfc7().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MUXSYNCCLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc1(&mut self) -> SYNCOVFC1_W<MUXSYNCCLR_SPEC, 0> {
        SYNCOVFC1_W::new(self)
    }
    #[doc = "Bit 1 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc2(&mut self) -> SYNCOVFC2_W<MUXSYNCCLR_SPEC, 1> {
        SYNCOVFC2_W::new(self)
    }
    #[doc = "Bit 2 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc3(&mut self) -> SYNCOVFC3_W<MUXSYNCCLR_SPEC, 2> {
        SYNCOVFC3_W::new(self)
    }
    #[doc = "Bit 3 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc4(&mut self) -> SYNCOVFC4_W<MUXSYNCCLR_SPEC, 3> {
        SYNCOVFC4_W::new(self)
    }
    #[doc = "Bit 4 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc5(&mut self) -> SYNCOVFC5_W<MUXSYNCCLR_SPEC, 4> {
        SYNCOVFC5_W::new(self)
    }
    #[doc = "Bit 5 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc6(&mut self) -> SYNCOVFC6_W<MUXSYNCCLR_SPEC, 5> {
        SYNCOVFC6_W::new(self)
    }
    #[doc = "Bit 6 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc7(&mut self) -> SYNCOVFC7_W<MUXSYNCCLR_SPEC, 6> {
        SYNCOVFC7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Interrupt Clear Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxsyncclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxsyncclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXSYNCCLR_SPEC;
impl crate::RegisterSpec for MUXSYNCCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxsyncclr::R`](R) reader structure"]
impl crate::Readable for MUXSYNCCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxsyncclr::W`](W) writer structure"]
impl crate::Writable for MUXSYNCCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXSYNCCLR to value 0"]
impl crate::Resettable for MUXSYNCCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
