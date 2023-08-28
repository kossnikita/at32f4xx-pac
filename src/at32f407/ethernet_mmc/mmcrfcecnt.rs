#[doc = "Register `MMCRFCECNT` reader"]
pub type R = crate::R<MMCRFCECNT_SPEC>;
#[doc = "Field `RFCEC` reader - Received frames CRC error counter"]
pub type RFCEC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received frames CRC error counter"]
    #[inline(always)]
    pub fn rfcec(&self) -> RFCEC_R {
        RFCEC_R::new(self.bits)
    }
}
#[doc = "Ethernet MMC received frames with CRC error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrfcecnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRFCECNT_SPEC;
impl crate::RegisterSpec for MMCRFCECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrfcecnt::R`](R) reader structure"]
impl crate::Readable for MMCRFCECNT_SPEC {}
#[doc = "`reset()` method sets MMCRFCECNT to value 0"]
impl crate::Resettable for MMCRFCECNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
