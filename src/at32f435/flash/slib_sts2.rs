#[doc = "Register `SLIB_STS2` reader"]
pub type R = crate::R<SLIB_STS2_SPEC>;
#[doc = "Register `SLIB_STS2` writer"]
pub type W = crate::W<SLIB_STS2_SPEC>;
#[doc = "Field `SLIB_INST_SS` reader - sLib instruction start sector"]
pub type SLIB_INST_SS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - sLib instruction start sector"]
    #[inline(always)]
    pub fn slib_inst_ss(&self) -> SLIB_INST_SS_R {
        SLIB_INST_SS_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLIB_STS2")
            .field(
                "slib_inst_ss",
                &format_args!("{}", self.slib_inst_ss().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SLIB_STS2_SPEC> {
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
#[doc = "sLib status 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_STS2_SPEC;
impl crate::RegisterSpec for SLIB_STS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_sts2::R`](R) reader structure"]
impl crate::Readable for SLIB_STS2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slib_sts2::W`](W) writer structure"]
impl crate::Writable for SLIB_STS2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLIB_STS2 to value 0xffff"]
impl crate::Resettable for SLIB_STS2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
