#[doc = "Register `DT14` reader"]
pub type R = crate::R<DT14_SPEC>;
#[doc = "Register `DT14` writer"]
pub type W = crate::W<DT14_SPEC>;
#[doc = "Field `DT14` reader - BPR data14"]
pub type DT14_R = crate::FieldReader<u16>;
#[doc = "Field `DT14` writer - BPR data14"]
pub type DT14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data14"]
    #[inline(always)]
    pub fn dt14(&self) -> DT14_R {
        DT14_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data14"]
    #[inline(always)]
    #[must_use]
    pub fn dt14(&mut self) -> DT14_W<DT14_SPEC, 0> {
        DT14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT14_SPEC;
impl crate::RegisterSpec for DT14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt14::R`](R) reader structure"]
impl crate::Readable for DT14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt14::W`](W) writer structure"]
impl crate::Writable for DT14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT14 to value 0"]
impl crate::Resettable for DT14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
