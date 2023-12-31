#[doc = "Register `TCRC` reader"]
pub type R = crate::R<TCRC_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TCRC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Transmit CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcrc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCRC_SPEC;
impl crate::RegisterSpec for TCRC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tcrc::R`](R) reader structure"]
impl crate::Readable for TCRC_SPEC {}
#[doc = "`reset()` method sets TCRC to value 0"]
impl crate::Resettable for TCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
