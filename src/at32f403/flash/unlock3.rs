#[doc = "Register `UNLOCK3` writer"]
pub type W = crate::W<UNLOCK3_SPEC>;
#[doc = "Field `UKVAL` writer - Unlock key value"]
pub type UKVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<UNLOCK3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Unlock key value"]
    #[inline(always)]
    #[must_use]
    pub fn ukval(&mut self) -> UKVAL_W<UNLOCK3_SPEC> {
        UKVAL_W::new(self, 0)
    }
}
#[doc = "Unlock 3 register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNLOCK3_SPEC;
impl crate::RegisterSpec for UNLOCK3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`unlock3::W`](W) writer structure"]
impl crate::Writable for UNLOCK3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UNLOCK3 to value 0"]
impl crate::Resettable for UNLOCK3_SPEC {
    const RESET_VALUE: u32 = 0;
}
