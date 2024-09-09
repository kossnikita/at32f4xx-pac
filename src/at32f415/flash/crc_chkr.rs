#[doc = "Register `CRC_CHKR` reader"]
pub type R = crate::R<CRC_CHKR_SPEC>;
#[doc = "Field `FCRC_OUT` reader - CRC32 verification result of flash user code or SLIB code"]
pub type FCRC_OUT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CRC32 verification result of flash user code or SLIB code"]
    #[inline(always)]
    pub fn fcrc_out(&self) -> FCRC_OUT_R {
        FCRC_OUT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC_CHKR")
            .field("fcrc_out", &self.fcrc_out())
            .finish()
    }
}
#[doc = "FLASH CRC check result register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_chkr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_CHKR_SPEC;
impl crate::RegisterSpec for CRC_CHKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_chkr::R`](R) reader structure"]
impl crate::Readable for CRC_CHKR_SPEC {}
#[doc = "`reset()` method sets CRC_CHKR to value 0"]
impl crate::Resettable for CRC_CHKR_SPEC {
    const RESET_VALUE: u32 = 0;
}
