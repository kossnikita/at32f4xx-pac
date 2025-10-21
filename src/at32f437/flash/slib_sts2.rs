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
            .field("slib_inst_ss", &self.slib_inst_ss())
            .finish()
    }
}
impl W {}
#[doc = "sLib status 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`slib_sts2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_sts2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_STS2_SPEC;
impl crate::RegisterSpec for SLIB_STS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_sts2::R`](R) reader structure"]
impl crate::Readable for SLIB_STS2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slib_sts2::W`](W) writer structure"]
impl crate::Writable for SLIB_STS2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLIB_STS2 to value 0xffff"]
impl crate::Resettable for SLIB_STS2_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
