#[doc = "Register `UNLOCK3` writer"]
pub type W = crate::W<UNLOCK3_SPEC>;
#[doc = "Field `UKVAL` writer - Unlock key value"]
pub type UKVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl core::fmt::Debug for crate::generic::Reg<UNLOCK3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Unlock key value"]
    #[inline(always)]
    #[must_use]
    pub fn ukval(&mut self) -> UKVAL_W<UNLOCK3_SPEC, 0> {
        UKVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Unlock 3 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unlock3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNLOCK3_SPEC;
impl crate::RegisterSpec for UNLOCK3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`unlock3::W`](W) writer structure"]
impl crate::Writable for UNLOCK3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UNLOCK3 to value 0"]
impl crate::Resettable for UNLOCK3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
