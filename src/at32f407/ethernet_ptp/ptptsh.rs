#[doc = "Register `PTPTSH` reader"]
pub type R = crate::R<PTPTSH_SPEC>;
#[doc = "Field `TS` reader - Timestamp second"]
pub type TS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp second"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSH")
            .field("ts", &format_args!("{}", self.ts().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PTPTSH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Ethernet PTP time stamp high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSH_SPEC;
impl crate::RegisterSpec for PTPTSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptsh::R`](R) reader structure"]
impl crate::Readable for PTPTSH_SPEC {}
#[doc = "`reset()` method sets PTPTSH to value 0"]
impl crate::Resettable for PTPTSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
