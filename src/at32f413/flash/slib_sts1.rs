#[doc = "Register `SLIB_STS1` reader"]
pub type R = crate::R<SLIB_STS1_SPEC>;
#[doc = "Register `SLIB_STS1` writer"]
pub type W = crate::W<SLIB_STS1_SPEC>;
#[doc = "Field `SLIB_SS` reader - sLib start sector"]
pub type SLIB_SS_R = crate::FieldReader<u16>;
#[doc = "Field `SLIB_DAT_SS` reader - sLib data start sector"]
pub type SLIB_DAT_SS_R = crate::FieldReader<u16>;
#[doc = "Field `SLIB_ES` reader - sLib end sector"]
pub type SLIB_ES_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - sLib start sector"]
    #[inline(always)]
    pub fn slib_ss(&self) -> SLIB_SS_R {
        SLIB_SS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - sLib data start sector"]
    #[inline(always)]
    pub fn slib_dat_ss(&self) -> SLIB_DAT_SS_R {
        SLIB_DAT_SS_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 22:31 - sLib end sector"]
    #[inline(always)]
    pub fn slib_es(&self) -> SLIB_ES_R {
        SLIB_ES_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLIB_STS1")
            .field("slib_ss", &format_args!("{}", self.slib_ss().bits()))
            .field(
                "slib_dat_ss",
                &format_args!("{}", self.slib_dat_ss().bits()),
            )
            .field("slib_es", &format_args!("{}", self.slib_es().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SLIB_STS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
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
#[doc = "sLib status 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_STS1_SPEC;
impl crate::RegisterSpec for SLIB_STS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_sts1::R`](R) reader structure"]
impl crate::Readable for SLIB_STS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slib_sts1::W`](W) writer structure"]
impl crate::Writable for SLIB_STS1_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLIB_STS1 to value 0"]
impl crate::Resettable for SLIB_STS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
