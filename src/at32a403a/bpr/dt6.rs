#[doc = "Register `DT6` reader"]
pub type R = crate::R<DT6_SPEC>;
#[doc = "Register `DT6` writer"]
pub type W = crate::W<DT6_SPEC>;
#[doc = "Field `DT6` reader - BPR data6"]
pub type DT6_R = crate::FieldReader<u16>;
#[doc = "Field `DT6` writer - BPR data6"]
pub type DT6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data6"]
    #[inline(always)]
    pub fn dt6(&self) -> DT6_R {
        DT6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data6"]
    #[inline(always)]
    #[must_use]
    pub fn dt6(&mut self) -> DT6_W<DT6_SPEC, 0> {
        DT6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT6_SPEC;
impl crate::RegisterSpec for DT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt6::R`](R) reader structure"]
impl crate::Readable for DT6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt6::W`](W) writer structure"]
impl crate::Writable for DT6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT6 to value 0"]
impl crate::Resettable for DT6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
