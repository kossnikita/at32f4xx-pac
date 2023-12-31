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
            .field("fcrc_out", &format_args!("{}", self.fcrc_out().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CRC_CHKR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "FLASH CRC check result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_chkr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
