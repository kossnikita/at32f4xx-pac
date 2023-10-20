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
    #[must_use]
    pub fn slib_pclr_val(&mut self) -> SLIB_PCLR_VAL_W<SLIB_PWD_CLR_SPEC> {
        SLIB_PCLR_VAL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SLIB password clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_pwd_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_PWD_CLR_SPEC;
impl crate::RegisterSpec for SLIB_PWD_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_pwd_clr::W`](W) writer structure"]
impl crate::Writable for SLIB_PWD_CLR_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLIB_PWD_CLR to value 0"]
impl crate::Resettable for SLIB_PWD_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
