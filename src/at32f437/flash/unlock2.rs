#[doc = "Register `UNLOCK2` writer"]
pub type W = crate::W<UNLOCK2_SPEC>;
#[doc = "Field `UKVAL` writer - Unlock key value"]
pub type UKVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<UNLOCK2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Unlock key value"]
    #[inline(always)]
    pub fn ukval(&mut self) -> UKVAL_W<'_, UNLOCK2_SPEC> {
        UKVAL_W::new(self, 0)
    }
}
#[doc = "Unlock 2 register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNLOCK2_SPEC;
impl crate::RegisterSpec for UNLOCK2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`unlock2::W`](W) writer structure"]
impl crate::Writable for UNLOCK2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNLOCK2 to value 0"]
impl crate::Resettable for UNLOCK2_SPEC {}
