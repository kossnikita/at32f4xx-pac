#[doc = "Register `RPR` reader"]
pub type R = crate::R<RPR_SPEC>;
#[doc = "Register `RPR` writer"]
pub type W = crate::W<RPR_SPEC>;
#[doc = "Field `RPR` reader - Repetition of period value"]
pub type RPR_R = crate::FieldReader;
#[doc = "Field `RPR` writer - Repetition of period value"]
pub type RPR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Repetition of period value"]
    #[inline(always)]
    pub fn rpr(&self) -> RPR_R {
        RPR_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPR")
            .field("rpr", &format_args!("{}", self.rpr().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RPR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repetition of period value"]
    #[inline(always)]
    #[must_use]
    pub fn rpr(&mut self) -> RPR_W<RPR_SPEC> {
        RPR_W::new(self, 0)
    }
}
#[doc = "Repetition of period value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPR_SPEC;
impl crate::RegisterSpec for RPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpr::R`](R) reader structure"]
impl crate::Readable for RPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rpr::W`](W) writer structure"]
impl crate::Writable for RPR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RPR to value 0"]
impl crate::Resettable for RPR_SPEC {
    const RESET_VALUE: u32 = 0;
}
