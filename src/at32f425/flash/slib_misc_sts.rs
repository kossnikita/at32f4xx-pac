#[doc = "Register `SLIB_MISC_STS` reader"]
pub type R = crate::R<SLIB_MISC_STS_SPEC>;
#[doc = "Register `SLIB_MISC_STS` writer"]
pub type W = crate::W<SLIB_MISC_STS_SPEC>;
#[doc = "Field `SLIB_PWD_ERR` reader - sLib password error"]
pub type SLIB_PWD_ERR_R = crate::BitReader;
#[doc = "Field `SLIB_PWD_OK` reader - sLib password ok"]
pub type SLIB_PWD_OK_R = crate::BitReader;
#[doc = "Field `SLIB_ULKF` reader - sLib unlock flag"]
pub type SLIB_ULKF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - sLib password error"]
    #[inline(always)]
    pub fn slib_pwd_err(&self) -> SLIB_PWD_ERR_R {
        SLIB_PWD_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sLib password ok"]
    #[inline(always)]
    pub fn slib_pwd_ok(&self) -> SLIB_PWD_OK_R {
        SLIB_PWD_OK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sLib unlock flag"]
    #[inline(always)]
    pub fn slib_ulkf(&self) -> SLIB_ULKF_R {
        SLIB_ULKF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLIB_MISC_STS")
            .field(
                "slib_pwd_err",
                &format_args!("{}", self.slib_pwd_err().bit()),
            )
            .field("slib_pwd_ok", &format_args!("{}", self.slib_pwd_ok().bit()))
            .field("slib_ulkf", &format_args!("{}", self.slib_ulkf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SLIB_MISC_STS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "sLib misc status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_misc_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_misc_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_MISC_STS_SPEC;
impl crate::RegisterSpec for SLIB_MISC_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_misc_sts::R`](R) reader structure"]
impl crate::Readable for SLIB_MISC_STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slib_misc_sts::W`](W) writer structure"]
impl crate::Writable for SLIB_MISC_STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLIB_MISC_STS to value 0x0100_0000"]
impl crate::Resettable for SLIB_MISC_STS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
