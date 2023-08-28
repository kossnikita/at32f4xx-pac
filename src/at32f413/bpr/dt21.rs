#[doc = "Register `DT21` reader"]
pub type R = crate::R<DT21_SPEC>;
#[doc = "Register `DT21` writer"]
pub type W = crate::W<DT21_SPEC>;
#[doc = "Field `DT21` reader - BPR data21"]
pub type DT21_R = crate::FieldReader<u16>;
#[doc = "Field `DT21` writer - BPR data21"]
pub type DT21_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data21"]
    #[inline(always)]
    pub fn dt21(&self) -> DT21_R {
        DT21_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data21"]
    #[inline(always)]
    #[must_use]
    pub fn dt21(&mut self) -> DT21_W<DT21_SPEC, 0> {
        DT21_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT21_SPEC;
impl crate::RegisterSpec for DT21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt21::R`](R) reader structure"]
impl crate::Readable for DT21_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt21::W`](W) writer structure"]
impl crate::Writable for DT21_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT21 to value 0"]
impl crate::Resettable for DT21_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
