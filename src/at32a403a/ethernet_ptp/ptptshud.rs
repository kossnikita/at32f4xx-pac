#[doc = "Register `PTPTSHUD` reader"]
pub type R = crate::R<PTPTSHUD_SPEC>;
#[doc = "Register `PTPTSHUD` writer"]
pub type W = crate::W<PTPTSHUD_SPEC>;
#[doc = "Field `TS` reader - Timestamp second"]
pub type TS_R = crate::FieldReader<u32>;
#[doc = "Field `TS` writer - Timestamp second"]
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp second"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSHUD").field("ts", &self.ts()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp second"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<PTPTSHUD_SPEC> {
        TS_W::new(self, 0)
    }
}
#[doc = "Ethernet PTP time stamp high update register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptshud::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptshud::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSHUD_SPEC;
impl crate::RegisterSpec for PTPTSHUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptshud::R`](R) reader structure"]
impl crate::Readable for PTPTSHUD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptptshud::W`](W) writer structure"]
impl crate::Writable for PTPTSHUD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPTSHUD to value 0"]
impl crate::Resettable for PTPTSHUD_SPEC {
    const RESET_VALUE: u32 = 0;
}
