#[doc = "Register `DT37` reader"]
pub type R = crate::R<DT37_SPEC>;
#[doc = "Register `DT37` writer"]
pub type W = crate::W<DT37_SPEC>;
#[doc = "Field `DT37` reader - BPR data37"]
pub type DT37_R = crate::FieldReader<u16>;
#[doc = "Field `DT37` writer - BPR data37"]
pub type DT37_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data37"]
    #[inline(always)]
    pub fn dt37(&self) -> DT37_R {
        DT37_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data37"]
    #[inline(always)]
    #[must_use]
    pub fn dt37(&mut self) -> DT37_W<DT37_SPEC, 0> {
        DT37_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt37::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt37::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT37_SPEC;
impl crate::RegisterSpec for DT37_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt37::R`](R) reader structure"]
impl crate::Readable for DT37_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt37::W`](W) writer structure"]
impl crate::Writable for DT37_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT37 to value 0"]
impl crate::Resettable for DT37_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
