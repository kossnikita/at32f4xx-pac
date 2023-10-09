#[doc = "Register `SLIB_UNLOCK` writer"]
pub type W = crate::W<SLIB_UNLOCK_SPEC>;
#[doc = "Field `SLIB_UKVAL` writer - sLib unlock key value"]
pub type SLIB_UKVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl core::fmt::Debug for crate::generic::Reg<SLIB_UNLOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - sLib unlock key value"]
    #[inline(always)]
    #[must_use]
    pub fn slib_ukval(&mut self) -> SLIB_UKVAL_W<SLIB_UNLOCK_SPEC, 0> {
        SLIB_UKVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "sLib unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_unlock::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_UNLOCK_SPEC;
impl crate::RegisterSpec for SLIB_UNLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_unlock::W`](W) writer structure"]
impl crate::Writable for SLIB_UNLOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLIB_UNLOCK to value 0"]
impl crate::Resettable for SLIB_UNLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
