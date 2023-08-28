#[doc = "Register `DT18` reader"]
pub type R = crate::R<DT18_SPEC>;
#[doc = "Register `DT18` writer"]
pub type W = crate::W<DT18_SPEC>;
#[doc = "Field `DT18` reader - BPR data18"]
pub type DT18_R = crate::FieldReader<u16>;
#[doc = "Field `DT18` writer - BPR data18"]
pub type DT18_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data18"]
    #[inline(always)]
    pub fn dt18(&self) -> DT18_R {
        DT18_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data18"]
    #[inline(always)]
    #[must_use]
    pub fn dt18(&mut self) -> DT18_W<DT18_SPEC, 0> {
        DT18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT18_SPEC;
impl crate::RegisterSpec for DT18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt18::R`](R) reader structure"]
impl crate::Readable for DT18_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt18::W`](W) writer structure"]
impl crate::Writable for DT18_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT18 to value 0"]
impl crate::Resettable for DT18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
