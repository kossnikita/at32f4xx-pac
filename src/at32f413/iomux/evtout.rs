#[doc = "Register `EVTOUT` reader"]
pub type R = crate::R<EVTOUT_SPEC>;
#[doc = "Register `EVTOUT` writer"]
pub type W = crate::W<EVTOUT_SPEC>;
#[doc = "Field `SELPIN` reader - Select pin"]
pub type SELPIN_R = crate::FieldReader;
#[doc = "Field `SELPIN` writer - Select pin"]
pub type SELPIN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SELPORT` reader - Select port"]
pub type SELPORT_R = crate::FieldReader;
#[doc = "Field `SELPORT` writer - Select port"]
pub type SELPORT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `EVOEN` reader - Event output enable"]
pub type EVOEN_R = crate::BitReader;
#[doc = "Field `EVOEN` writer - Event output enable"]
pub type EVOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Select pin"]
    #[inline(always)]
    pub fn selpin(&self) -> SELPIN_R {
        SELPIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Select port"]
    #[inline(always)]
    pub fn selport(&self) -> SELPORT_R {
        SELPORT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    pub fn evoen(&self) -> EVOEN_R {
        EVOEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select pin"]
    #[inline(always)]
    #[must_use]
    pub fn selpin(&mut self) -> SELPIN_W<EVTOUT_SPEC, 0> {
        SELPIN_W::new(self)
    }
    #[doc = "Bits 4:6 - Select port"]
    #[inline(always)]
    #[must_use]
    pub fn selport(&mut self) -> SELPORT_W<EVTOUT_SPEC, 4> {
        SELPORT_W::new(self)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    #[must_use]
    pub fn evoen(&mut self) -> EVOEN_W<EVTOUT_SPEC, 7> {
        EVOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Event output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVTOUT_SPEC;
impl crate::RegisterSpec for EVTOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtout::R`](R) reader structure"]
impl crate::Readable for EVTOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evtout::W`](W) writer structure"]
impl crate::Writable for EVTOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVTOUT to value 0"]
impl crate::Resettable for EVTOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
