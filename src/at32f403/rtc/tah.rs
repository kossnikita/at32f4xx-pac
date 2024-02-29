#[doc = "Register `TAH` writer"]
pub type W = crate::W<TAH_SPEC>;
#[doc = "Field `TA` writer - Time alarm register high"]
pub type TA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<TAH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Time alarm register high"]
    #[inline(always)]
    #[must_use]
    pub fn ta(&mut self) -> TA_W<TAH_SPEC> {
        TA_W::new(self, 0)
    }
}
#[doc = "RTC Alarm Register High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tah::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAH_SPEC;
impl crate::RegisterSpec for TAH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tah::W`](W) writer structure"]
impl crate::Writable for TAH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAH to value 0xffff"]
impl crate::Resettable for TAH_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
