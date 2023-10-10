#[doc = "Register `TAL` writer"]
pub type W = crate::W<TAL_SPEC>;
#[doc = "Field `TA` writer - RTC alarm register low"]
pub type TA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl core::fmt::Debug for crate::generic::Reg<TAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC alarm register low"]
    #[inline(always)]
    #[must_use]
    pub fn ta(&mut self) -> TA_W<TAL_SPEC, 0> {
        TA_W::new(self)
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
#[doc = "Time alarm register low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tal::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAL_SPEC;
impl crate::RegisterSpec for TAL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tal::W`](W) writer structure"]
impl crate::Writable for TAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAL to value 0xffff"]
impl crate::Resettable for TAL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
