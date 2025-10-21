#[doc = "Register `DIVH` writer"]
pub type W = crate::W<DIVH_SPEC>;
#[doc = "Field `DIV` writer - RTC divider high"]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl core::fmt::Debug for crate::generic::Reg<DIVH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:3 - RTC divider high"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<'_, DIVH_SPEC> {
        DIV_W::new(self, 0)
    }
}
#[doc = "RTC Divider Register High\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVH_SPEC;
impl crate::RegisterSpec for DIVH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`divh::W`](W) writer structure"]
impl crate::Writable for DIVH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIVH to value 0"]
impl crate::Resettable for DIVH_SPEC {}
