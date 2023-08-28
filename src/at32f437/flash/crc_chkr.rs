#[doc = "Register `CRC_CHKR` reader"]
pub type R = crate::R<CRC_CHKR_SPEC>;
#[doc = "Field `CRC_CHKR` reader - CRC check result"]
pub type CRC_CHKR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CRC check result"]
    #[inline(always)]
    pub fn crc_chkr(&self) -> CRC_CHKR_R {
        CRC_CHKR_R::new(self.bits)
    }
}
#[doc = "CRC check result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_chkr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_CHKR_SPEC;
impl crate::RegisterSpec for CRC_CHKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_chkr::R`](R) reader structure"]
impl crate::Readable for CRC_CHKR_SPEC {}
#[doc = "`reset()` method sets CRC_CHKR to value 0"]
impl crate::Resettable for CRC_CHKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
