#[doc = "Register `CTRLH` reader"]
pub type R = crate::R<CTRLH_SPEC>;
#[doc = "Register `CTRLH` writer"]
pub type W = crate::W<CTRLH_SPEC>;
#[doc = "Field `OVFIEN` reader - Overflow interrupt enable"]
pub type OVFIEN_R = crate::BitReader;
#[doc = "Field `OVFIEN` writer - Overflow interrupt enable"]
pub type OVFIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAIEN` reader - Time alarm interrupt enable"]
pub type TAIEN_R = crate::BitReader;
#[doc = "Field `TAIEN` writer - Time alarm interrupt enable"]
pub type TAIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSIEN` reader - Time second interrupt enable"]
pub type TSIEN_R = crate::BitReader;
#[doc = "Field `TSIEN` writer - Time second interrupt enable"]
pub type TSIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&self) -> OVFIEN_R {
        OVFIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time alarm interrupt enable"]
    #[inline(always)]
    pub fn taien(&self) -> TAIEN_R {
        TAIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time second interrupt enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TSIEN_R {
        TSIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfien(&mut self) -> OVFIEN_W<CTRLH_SPEC, 0> {
        OVFIEN_W::new(self)
    }
    #[doc = "Bit 1 - Time alarm interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn taien(&mut self) -> TAIEN_W<CTRLH_SPEC, 1> {
        TAIEN_W::new(self)
    }
    #[doc = "Bit 2 - Time second interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsien(&mut self) -> TSIEN_W<CTRLH_SPEC, 2> {
        TSIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC Control Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLH_SPEC;
impl crate::RegisterSpec for CTRLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlh::R`](R) reader structure"]
impl crate::Readable for CTRLH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlh::W`](W) writer structure"]
impl crate::Writable for CTRLH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLH to value 0"]
impl crate::Resettable for CTRLH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
