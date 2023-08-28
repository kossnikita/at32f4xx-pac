#[doc = "Register `RPR` reader"]
pub type R = crate::R<RPR_SPEC>;
#[doc = "Register `RPR` writer"]
pub type W = crate::W<RPR_SPEC>;
#[doc = "Field `RPR` reader - Repetition of period value"]
pub type RPR_R = crate::FieldReader;
#[doc = "Field `RPR` writer - Repetition of period value"]
pub type RPR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Repetition of period value"]
    #[inline(always)]
    pub fn rpr(&self) -> RPR_R {
        RPR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repetition of period value"]
    #[inline(always)]
    #[must_use]
    pub fn rpr(&mut self) -> RPR_W<RPR_SPEC, 0> {
        RPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RPR to value 0"]
impl crate::Resettable for RPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
