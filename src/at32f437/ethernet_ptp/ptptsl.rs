#[doc = "Register `PTPTSL` reader"]
pub type R = crate::R<PTPTSL_SPEC>;
#[doc = "Field `TSS` reader - Timestamp subseconds"]
pub type TSS_R = crate::FieldReader<u32>;
#[doc = "Field `AST` reader - Add or subtract time"]
pub type AST_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - Timestamp subseconds"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Add or subtract time"]
    #[inline(always)]
    pub fn ast(&self) -> AST_R {
        AST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSL")
            .field("tss", &self.tss())
            .field("ast", &self.ast())
            .finish()
    }
}
#[doc = "Ethernet PTP time stamp low register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptsl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSL_SPEC;
impl crate::RegisterSpec for PTPTSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptsl::R`](R) reader structure"]
impl crate::Readable for PTPTSL_SPEC {}
#[doc = "`reset()` method sets PTPTSL to value 0"]
impl crate::Resettable for PTPTSL_SPEC {
    const RESET_VALUE: u32 = 0;
}
