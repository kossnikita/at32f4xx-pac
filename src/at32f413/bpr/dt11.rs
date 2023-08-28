#[doc = "Register `DT11` reader"]
pub type R = crate::R<DT11_SPEC>;
#[doc = "Register `DT11` writer"]
pub type W = crate::W<DT11_SPEC>;
#[doc = "Field `DT11` reader - BPR data11"]
pub type DT11_R = crate::FieldReader<u16>;
#[doc = "Field `DT11` writer - BPR data11"]
pub type DT11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data11"]
    #[inline(always)]
    pub fn dt11(&self) -> DT11_R {
        DT11_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data11"]
    #[inline(always)]
    #[must_use]
    pub fn dt11(&mut self) -> DT11_W<DT11_SPEC, 0> {
        DT11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT11_SPEC;
impl crate::RegisterSpec for DT11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt11::R`](R) reader structure"]
impl crate::Readable for DT11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt11::W`](W) writer structure"]
impl crate::Writable for DT11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT11 to value 0"]
impl crate::Resettable for DT11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
