#[doc = "Register `MUXC6CTRL` reader"]
pub type R = crate::R<MUXC6CTRL_SPEC>;
#[doc = "Register `MUXC6CTRL` writer"]
pub type W = crate::W<MUXC6CTRL_SPEC>;
#[doc = "Field `REQSEL` reader - DMA request select"]
pub type REQSEL_R = crate::FieldReader;
#[doc = "Field `REQSEL` writer - DMA request select"]
pub type REQSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `SYNCOVIEN` reader - Synchronization overrun interrupt enable"]
pub type SYNCOVIEN_R = crate::BitReader;
#[doc = "Field `SYNCOVIEN` writer - Synchronization overrun interrupt enable"]
pub type SYNCOVIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVTGEN` reader - Event generation enable"]
pub type EVTGEN_R = crate::BitReader;
#[doc = "Field `EVTGEN` writer - Event generation enable"]
pub type EVTGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNCEN` reader - Synchroniztion enable"]
pub type SYNCEN_R = crate::BitReader;
#[doc = "Field `SYNCEN` writer - Synchroniztion enable"]
pub type SYNCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNCPOL` reader - Synchronization polarity"]
pub type SYNCPOL_R = crate::FieldReader;
#[doc = "Field `SYNCPOL` writer - Synchronization polarity"]
pub type SYNCPOL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `REQCNT` reader - Number of DMA requests"]
pub type REQCNT_R = crate::FieldReader;
#[doc = "Field `REQCNT` writer - Number of DMA requests"]
pub type REQCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SYNCSEL` reader - Synchronization Identification"]
pub type SYNCSEL_R = crate::FieldReader;
#[doc = "Field `SYNCSEL` writer - Synchronization Identification"]
pub type SYNCSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:6 - DMA request select"]
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn syncovien(&self) -> SYNCOVIEN_R {
        SYNCOVIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    pub fn evtgen(&self) -> EVTGEN_R {
        EVTGEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchroniztion enable"]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization polarity"]
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests"]
    #[inline(always)]
    pub fn reqcnt(&self) -> REQCNT_R {
        REQCNT_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization Identification"]
    #[inline(always)]
    pub fn syncsel(&self) -> SYNCSEL_R {
        SYNCSEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA request select"]
    #[inline(always)]
    #[must_use]
    pub fn reqsel(&mut self) -> REQSEL_W<MUXC6CTRL_SPEC, 0> {
        REQSEL_W::new(self)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncovien(&mut self) -> SYNCOVIEN_W<MUXC6CTRL_SPEC, 8> {
        SYNCOVIEN_W::new(self)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn evtgen(&mut self) -> EVTGEN_W<MUXC6CTRL_SPEC, 9> {
        EVTGEN_W::new(self)
    }
    #[doc = "Bit 16 - Synchroniztion enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SYNCEN_W<MUXC6CTRL_SPEC, 16> {
        SYNCEN_W::new(self)
    }
    #[doc = "Bits 17:18 - Synchronization polarity"]
    #[inline(always)]
    #[must_use]
    pub fn syncpol(&mut self) -> SYNCPOL_W<MUXC6CTRL_SPEC, 17> {
        SYNCPOL_W::new(self)
    }
    #[doc = "Bits 19:23 - Number of DMA requests"]
    #[inline(always)]
    #[must_use]
    pub fn reqcnt(&mut self) -> REQCNT_W<MUXC6CTRL_SPEC, 19> {
        REQCNT_W::new(self)
    }
    #[doc = "Bits 24:28 - Synchronization Identification"]
    #[inline(always)]
    #[must_use]
    pub fn syncsel(&mut self) -> SYNCSEL_W<MUXC6CTRL_SPEC, 24> {
        SYNCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel 6 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc6ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc6ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXC6CTRL_SPEC;
impl crate::RegisterSpec for MUXC6CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxc6ctrl::R`](R) reader structure"]
impl crate::Readable for MUXC6CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxc6ctrl::W`](W) writer structure"]
impl crate::Writable for MUXC6CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXC6CTRL to value 0"]
impl crate::Resettable for MUXC6CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
