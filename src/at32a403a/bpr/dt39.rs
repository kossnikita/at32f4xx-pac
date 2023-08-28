#[doc = "Register `DT39` reader"]
pub type R = crate::R<DT39_SPEC>;
#[doc = "Register `DT39` writer"]
pub type W = crate::W<DT39_SPEC>;
#[doc = "Field `DT39` reader - BPR data39"]
pub type DT39_R = crate::FieldReader<u16>;
#[doc = "Field `DT39` writer - BPR data39"]
pub type DT39_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data39"]
    #[inline(always)]
    pub fn dt39(&self) -> DT39_R {
        DT39_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data39"]
    #[inline(always)]
    #[must_use]
    pub fn dt39(&mut self) -> DT39_W<DT39_SPEC, 0> {
        DT39_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT39_SPEC;
impl crate::RegisterSpec for DT39_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt39::R`](R) reader structure"]
impl crate::Readable for DT39_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt39::W`](W) writer structure"]
impl crate::Writable for DT39_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT39 to value 0"]
impl crate::Resettable for DT39_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
