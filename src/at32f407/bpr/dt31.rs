#[doc = "Register `DT31` reader"]
pub type R = crate::R<DT31_SPEC>;
#[doc = "Register `DT31` writer"]
pub type W = crate::W<DT31_SPEC>;
#[doc = "Field `DT31` reader - BPR data31"]
pub type DT31_R = crate::FieldReader<u16>;
#[doc = "Field `DT31` writer - BPR data31"]
pub type DT31_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data31"]
    #[inline(always)]
    pub fn dt31(&self) -> DT31_R {
        DT31_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data31"]
    #[inline(always)]
    #[must_use]
    pub fn dt31(&mut self) -> DT31_W<DT31_SPEC, 0> {
        DT31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT31_SPEC;
impl crate::RegisterSpec for DT31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt31::R`](R) reader structure"]
impl crate::Readable for DT31_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt31::W`](W) writer structure"]
impl crate::Writable for DT31_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT31 to value 0"]
impl crate::Resettable for DT31_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
