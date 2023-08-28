#[doc = "Register `DT30` reader"]
pub type R = crate::R<DT30_SPEC>;
#[doc = "Register `DT30` writer"]
pub type W = crate::W<DT30_SPEC>;
#[doc = "Field `DT30` reader - BPR data30"]
pub type DT30_R = crate::FieldReader<u16>;
#[doc = "Field `DT30` writer - BPR data30"]
pub type DT30_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data30"]
    #[inline(always)]
    pub fn dt30(&self) -> DT30_R {
        DT30_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data30"]
    #[inline(always)]
    #[must_use]
    pub fn dt30(&mut self) -> DT30_W<DT30_SPEC, 0> {
        DT30_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT30_SPEC;
impl crate::RegisterSpec for DT30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt30::R`](R) reader structure"]
impl crate::Readable for DT30_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt30::W`](W) writer structure"]
impl crate::Writable for DT30_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT30 to value 0"]
impl crate::Resettable for DT30_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
