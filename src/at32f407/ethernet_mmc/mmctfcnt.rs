#[doc = "Register `MMCTFCNT` reader"]
pub type R = crate::R<MMCTFCNT_SPEC>;
#[doc = "Field `TGFC` reader - Transmitted good frames counter"]
pub type TGFC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames counter"]
    #[inline(always)]
    pub fn tgfc(&self) -> TGFC_R {
        TGFC_R::new(self.bits)
    }
}
#[doc = "Ethernet MMC transmitted good frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctfcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTFCNT_SPEC;
impl crate::RegisterSpec for MMCTFCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctfcnt::R`](R) reader structure"]
impl crate::Readable for MMCTFCNT_SPEC {}
#[doc = "`reset()` method sets MMCTFCNT to value 0"]
impl crate::Resettable for MMCTFCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
