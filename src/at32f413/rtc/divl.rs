#[doc = "Register `DIVL` writer"]
pub type W = crate::W<DIVL_SPEC>;
#[doc = "Field `DIV` writer - RTC divider low"]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<DIVL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC divider low"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<DIVL_SPEC> {
        DIV_W::new(self, 0)
    }
}
#[doc = "RTC Divider Register Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVL_SPEC;
impl crate::RegisterSpec for DIVL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`divl::W`](W) writer structure"]
impl crate::Writable for DIVL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIVL to value 0x8000"]
impl crate::Resettable for DIVL_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
