#[doc = "Register `TAL` writer"]
pub type W = crate::W<TAL_SPEC>;
#[doc = "Field `TA` writer - RTC alarm register low"]
pub type TA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<TAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC alarm register low"]
    #[inline(always)]
    #[must_use]
    pub fn ta(&mut self) -> TA_W<TAL_SPEC> {
        TA_W::new(self, 0)
    }
}
#[doc = "Time alarm register low\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tal::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAL_SPEC;
impl crate::RegisterSpec for TAL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tal::W`](W) writer structure"]
impl crate::Writable for TAL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAL to value 0xffff"]
impl crate::Resettable for TAL_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
