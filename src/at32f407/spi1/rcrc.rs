#[doc = "Register `RCRC` reader"]
pub type R = crate::R<RCRC_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Receive CRC register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcrc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCRC_SPEC;
impl crate::RegisterSpec for RCRC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rcrc::R`](R) reader structure"]
impl crate::Readable for RCRC_SPEC {}
#[doc = "`reset()` method sets RCRC to value 0"]
impl crate::Resettable for RCRC_SPEC {
    const RESET_VALUE: u16 = 0;
}
