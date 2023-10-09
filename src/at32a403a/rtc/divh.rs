#[doc = "Register `DIVH` writer"]
pub type W = crate::W<DIVH_SPEC>;
#[doc = "Field `DIV` writer - RTC divider high"]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl core::fmt::Debug for crate::generic::Reg<DIVH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:3 - RTC divider high"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<DIVH_SPEC, 0> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC Divider Register High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVH_SPEC;
impl crate::RegisterSpec for DIVH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`divh::W`](W) writer structure"]
impl crate::Writable for DIVH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIVH to value 0"]
impl crate::Resettable for DIVH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
