#[doc = "Register `DT13` reader"]
pub type R = crate::R<DT13_SPEC>;
#[doc = "Register `DT13` writer"]
pub type W = crate::W<DT13_SPEC>;
#[doc = "Field `DT13` reader - BPR data13"]
pub type DT13_R = crate::FieldReader<u16>;
#[doc = "Field `DT13` writer - BPR data13"]
pub type DT13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data13"]
    #[inline(always)]
    pub fn dt13(&self) -> DT13_R {
        DT13_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data13"]
    #[inline(always)]
    #[must_use]
    pub fn dt13(&mut self) -> DT13_W<DT13_SPEC, 0> {
        DT13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT13_SPEC;
impl crate::RegisterSpec for DT13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt13::R`](R) reader structure"]
impl crate::Readable for DT13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt13::W`](W) writer structure"]
impl crate::Writable for DT13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT13 to value 0"]
impl crate::Resettable for DT13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
