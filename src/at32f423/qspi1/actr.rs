#[doc = "Register `ACTR` reader"]
pub type R = crate::R<ACTR_SPEC>;
#[doc = "Register `ACTR` writer"]
pub type W = crate::W<ACTR_SPEC>;
#[doc = "Field `CSDLY` reader - CS delay"]
pub type CSDLY_R = crate::FieldReader;
#[doc = "Field `CSDLY` writer - CS delay"]
pub type CSDLY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - CS delay"]
    #[inline(always)]
    pub fn csdly(&self) -> CSDLY_R {
        CSDLY_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CS delay"]
    #[inline(always)]
    #[must_use]
    pub fn csdly(&mut self) -> CSDLY_W<ACTR_SPEC, 0> {
        CSDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AC timing control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACTR_SPEC;
impl crate::RegisterSpec for ACTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actr::R`](R) reader structure"]
impl crate::Readable for ACTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`actr::W`](W) writer structure"]
impl crate::Writable for ACTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTR to value 0x0f"]
impl crate::Resettable for ACTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}