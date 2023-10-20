#[doc = "Register `SLIB_UNLOCK` writer"]
pub type W = crate::W<SLIB_UNLOCK_SPEC>;
#[doc = "Field `SLIB_UKVAL` writer - sLib unlock key value"]
pub type SLIB_UKVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SLIB_UNLOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - sLib unlock key value"]
    #[inline(always)]
    #[must_use]
    pub fn slib_ukval(&mut self) -> SLIB_UKVAL_W<SLIB_UNLOCK_SPEC> {
        SLIB_UKVAL_W::new(self, 0)
    }
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
#[doc = "sLib unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_unlock::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_UNLOCK_SPEC;
impl crate::RegisterSpec for SLIB_UNLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_unlock::W`](W) writer structure"]
impl crate::Writable for SLIB_UNLOCK_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLIB_UNLOCK to value 0"]
impl crate::Resettable for SLIB_UNLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
