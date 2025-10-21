#[doc = "Register `SLIB_PWD_CLR` writer"]
pub type W = crate::W<SLIB_PWD_CLR_SPEC>;
#[doc = "Field `SLIB_PCLR_VAL` writer - sLib password clear value"]
pub type SLIB_PCLR_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SLIB_PWD_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - sLib password clear value"]
    #[inline(always)]
    pub fn slib_pclr_val(&mut self) -> SLIB_PCLR_VAL_W<'_, SLIB_PWD_CLR_SPEC> {
        SLIB_PCLR_VAL_W::new(self, 0)
    }
}
#[doc = "SLIB password clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_pwd_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_PWD_CLR_SPEC;
impl crate::RegisterSpec for SLIB_PWD_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_pwd_clr::W`](W) writer structure"]
impl crate::Writable for SLIB_PWD_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLIB_PWD_CLR to value 0"]
impl crate::Resettable for SLIB_PWD_CLR_SPEC {}
