#[doc = "Register `DT16` reader"]
pub type R = crate::R<DT16_SPEC>;
#[doc = "Register `DT16` writer"]
pub type W = crate::W<DT16_SPEC>;
#[doc = "Field `DT16` reader - BPR data16"]
pub type DT16_R = crate::FieldReader<u16>;
#[doc = "Field `DT16` writer - BPR data16"]
pub type DT16_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data16"]
    #[inline(always)]
    pub fn dt16(&self) -> DT16_R {
        DT16_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data16"]
    #[inline(always)]
    #[must_use]
    pub fn dt16(&mut self) -> DT16_W<DT16_SPEC, 0> {
        DT16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT16_SPEC;
impl crate::RegisterSpec for DT16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt16::R`](R) reader structure"]
impl crate::Readable for DT16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt16::W`](W) writer structure"]
impl crate::Writable for DT16_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT16 to value 0"]
impl crate::Resettable for DT16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
