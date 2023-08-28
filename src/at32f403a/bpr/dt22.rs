#[doc = "Register `DT22` reader"]
pub type R = crate::R<DT22_SPEC>;
#[doc = "Register `DT22` writer"]
pub type W = crate::W<DT22_SPEC>;
#[doc = "Field `DT22` reader - BPR data22"]
pub type DT22_R = crate::FieldReader<u16>;
#[doc = "Field `DT22` writer - BPR data22"]
pub type DT22_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data22"]
    #[inline(always)]
    pub fn dt22(&self) -> DT22_R {
        DT22_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data22"]
    #[inline(always)]
    #[must_use]
    pub fn dt22(&mut self) -> DT22_W<DT22_SPEC, 0> {
        DT22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT22_SPEC;
impl crate::RegisterSpec for DT22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt22::R`](R) reader structure"]
impl crate::Readable for DT22_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt22::W`](W) writer structure"]
impl crate::Writable for DT22_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT22 to value 0"]
impl crate::Resettable for DT22_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
