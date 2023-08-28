#[doc = "Register `DT42` reader"]
pub type R = crate::R<DT42_SPEC>;
#[doc = "Register `DT42` writer"]
pub type W = crate::W<DT42_SPEC>;
#[doc = "Field `DT42` reader - BPR data42"]
pub type DT42_R = crate::FieldReader<u16>;
#[doc = "Field `DT42` writer - BPR data42"]
pub type DT42_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data42"]
    #[inline(always)]
    pub fn dt42(&self) -> DT42_R {
        DT42_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data42"]
    #[inline(always)]
    #[must_use]
    pub fn dt42(&mut self) -> DT42_W<DT42_SPEC, 0> {
        DT42_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt42::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt42::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT42_SPEC;
impl crate::RegisterSpec for DT42_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt42::R`](R) reader structure"]
impl crate::Readable for DT42_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt42::W`](W) writer structure"]
impl crate::Writable for DT42_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT42 to value 0"]
impl crate::Resettable for DT42_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
