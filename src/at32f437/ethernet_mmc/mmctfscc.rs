#[doc = "Register `MMCTFSCC` reader"]
pub type R = crate::R<MMCTFSCC_SPEC>;
#[doc = "Field `TGFSCC` reader - Transmitted good frames single collision counter"]
pub type TGFSCC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames single collision counter"]
    #[inline(always)]
    pub fn tgfscc(&self) -> TGFSCC_R {
        TGFSCC_R::new(self.bits)
    }
}
#[doc = "Ethernet MMC transmitted good frames after a single collision counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctfscc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTFSCC_SPEC;
impl crate::RegisterSpec for MMCTFSCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctfscc::R`](R) reader structure"]
impl crate::Readable for MMCTFSCC_SPEC {}
#[doc = "`reset()` method sets MMCTFSCC to value 0"]
impl crate::Resettable for MMCTFSCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
