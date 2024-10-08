#[doc = "Register `SLIB_SET_PWD` writer"]
pub type W = crate::W<SLIB_SET_PWD_SPEC>;
#[doc = "Field `SLIB_PSET_VAL` writer - sLib password setting val"]
pub type SLIB_PSET_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SLIB_SET_PWD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - sLib password setting val"]
    #[inline(always)]
    #[must_use]
    pub fn slib_pset_val(&mut self) -> SLIB_PSET_VAL_W<SLIB_SET_PWD_SPEC> {
        SLIB_PSET_VAL_W::new(self, 0)
    }
}
#[doc = "sLib password setting register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_set_pwd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_SET_PWD_SPEC;
impl crate::RegisterSpec for SLIB_SET_PWD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_set_pwd::W`](W) writer structure"]
impl crate::Writable for SLIB_SET_PWD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLIB_SET_PWD to value 0"]
impl crate::Resettable for SLIB_SET_PWD_SPEC {
    const RESET_VALUE: u32 = 0;
}
