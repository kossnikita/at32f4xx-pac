#[doc = "Register `CODT` reader"]
pub type R = crate::R<CODT_SPEC>;
#[doc = "Field `CODTL` reader - Ordinary conversion low halfword data for master slave mode"]
pub type CODTL_R = crate::FieldReader<u16>;
#[doc = "Field `CODTH` reader - Ordinary conversion high halfword data for master slave mode"]
pub type CODTH_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Ordinary conversion low halfword data for master slave mode"]
    #[inline(always)]
    pub fn codtl(&self) -> CODTL_R {
        CODTL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Ordinary conversion high halfword data for master slave mode"]
    #[inline(always)]
    pub fn codth(&self) -> CODTH_R {
        CODTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CODT")
            .field("codth", &self.codth())
            .field("codtl", &self.codtl())
            .finish()
    }
}
#[doc = "Common Ordinary data register\n\nYou can [`read`](crate::Reg::read) this register and get [`codt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CODT_SPEC;
impl crate::RegisterSpec for CODT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codt::R`](R) reader structure"]
impl crate::Readable for CODT_SPEC {}
#[doc = "`reset()` method sets CODT to value 0"]
impl crate::Resettable for CODT_SPEC {
    const RESET_VALUE: u32 = 0;
}
