#[doc = "Register `MMCRGUFCNT` reader"]
pub type R = crate::R<MMCRGUFCNT_SPEC>;
#[doc = "Field `RGUFC` reader - Received good unicast frames counter"]
pub type RGUFC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received good unicast frames counter"]
    #[inline(always)]
    pub fn rgufc(&self) -> RGUFC_R {
        RGUFC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCRGUFCNT")
            .field("rgufc", &format_args!("{}", self.rgufc().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MMCRGUFCNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "MMC received good unicast frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrgufcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRGUFCNT_SPEC;
impl crate::RegisterSpec for MMCRGUFCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrgufcnt::R`](R) reader structure"]
impl crate::Readable for MMCRGUFCNT_SPEC {}
#[doc = "`reset()` method sets MMCRGUFCNT to value 0"]
impl crate::Resettable for MMCRGUFCNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
