#[doc = "Register `SLIB_SET_PWD` writer"]
pub type W = crate::W<SLIB_SET_PWD_SPEC>;
#[doc = "Field `SLIB_PSET_VAL` writer - sLib password setting val"]
pub type SLIB_PSET_VAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - sLib password setting val"]
    #[inline(always)]
    #[must_use]
    pub fn slib_pset_val(&mut self) -> SLIB_PSET_VAL_W<SLIB_SET_PWD_SPEC, 0> {
        SLIB_PSET_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "sLib password setting register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_pwd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_SET_PWD_SPEC;
impl crate::RegisterSpec for SLIB_SET_PWD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_set_pwd::W`](W) writer structure"]
impl crate::Writable for SLIB_SET_PWD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLIB_SET_PWD to value 0"]
impl crate::Resettable for SLIB_SET_PWD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
