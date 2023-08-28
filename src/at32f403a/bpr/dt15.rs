#[doc = "Register `DT15` reader"]
pub type R = crate::R<DT15_SPEC>;
#[doc = "Register `DT15` writer"]
pub type W = crate::W<DT15_SPEC>;
#[doc = "Field `DT15` reader - BPR data15"]
pub type DT15_R = crate::FieldReader<u16>;
#[doc = "Field `DT15` writer - BPR data15"]
pub type DT15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data15"]
    #[inline(always)]
    pub fn dt15(&self) -> DT15_R {
        DT15_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data15"]
    #[inline(always)]
    #[must_use]
    pub fn dt15(&mut self) -> DT15_W<DT15_SPEC, 0> {
        DT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT15_SPEC;
impl crate::RegisterSpec for DT15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt15::R`](R) reader structure"]
impl crate::Readable for DT15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt15::W`](W) writer structure"]
impl crate::Writable for DT15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT15 to value 0"]
impl crate::Resettable for DT15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
