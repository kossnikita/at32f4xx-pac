#[doc = "Register `REMAP2` writer"]
pub type W = crate::W<REMAP2_SPEC>;
#[doc = "Field `CMP_MUX` writer - CMP internal muxing"]
pub type CMP_MUX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl core::fmt::Debug for crate::generic::Reg<REMAP2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 26:27 - CMP internal muxing"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_mux(&mut self) -> CMP_MUX_W<REMAP2_SPEC> {
        CMP_MUX_W::new(self, 26)
    }
}
#[doc = "IO MUX remap register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP2_SPEC;
impl crate::RegisterSpec for REMAP2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`remap2::W`](W) writer structure"]
impl crate::Writable for REMAP2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP2 to value 0"]
impl crate::Resettable for REMAP2_SPEC {
    const RESET_VALUE: u32 = 0;
}
