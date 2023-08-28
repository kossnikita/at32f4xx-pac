#[doc = "Register `DT28` reader"]
pub type R = crate::R<DT28_SPEC>;
#[doc = "Register `DT28` writer"]
pub type W = crate::W<DT28_SPEC>;
#[doc = "Field `DT28` reader - BPR data28"]
pub type DT28_R = crate::FieldReader<u16>;
#[doc = "Field `DT28` writer - BPR data28"]
pub type DT28_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data28"]
    #[inline(always)]
    pub fn dt28(&self) -> DT28_R {
        DT28_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data28"]
    #[inline(always)]
    #[must_use]
    pub fn dt28(&mut self) -> DT28_W<DT28_SPEC, 0> {
        DT28_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT28_SPEC;
impl crate::RegisterSpec for DT28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt28::R`](R) reader structure"]
impl crate::Readable for DT28_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt28::W`](W) writer structure"]
impl crate::Writable for DT28_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT28 to value 0"]
impl crate::Resettable for DT28_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
