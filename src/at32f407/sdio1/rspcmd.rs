#[doc = "Register `RSPCMD` reader"]
pub type R = crate::R<RSPCMD_SPEC>;
#[doc = "Field `RSPCMD` reader - RSPCMD"]
pub type RSPCMD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - RSPCMD"]
    #[inline(always)]
    pub fn rspcmd(&self) -> RSPCMD_R {
        RSPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSPCMD")
            .field("rspcmd", &format_args!("{}", self.rspcmd().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RSPCMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SDIO command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspcmd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSPCMD_SPEC;
impl crate::RegisterSpec for RSPCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rspcmd::R`](R) reader structure"]
impl crate::Readable for RSPCMD_SPEC {}
#[doc = "`reset()` method sets RSPCMD to value 0"]
impl crate::Resettable for RSPCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
