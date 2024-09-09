#[doc = "Register `IDCODE` reader"]
pub type R = crate::R<IDCODE_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "DEBUG IDCODE\n\nYou can [`read`](crate::Reg::read) this register and get [`idcode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDCODE_SPEC;
impl crate::RegisterSpec for IDCODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idcode::R`](R) reader structure"]
impl crate::Readable for IDCODE_SPEC {}
#[doc = "`reset()` method sets IDCODE to value 0"]
impl crate::Resettable for IDCODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
