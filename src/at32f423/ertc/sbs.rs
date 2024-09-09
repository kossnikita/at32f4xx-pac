#[doc = "Register `SBS` reader"]
pub type R = crate::R<SBS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`sbs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SBS_SPEC;
impl crate::RegisterSpec for SBS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sbs::R`](R) reader structure"]
impl crate::Readable for SBS_SPEC {}
#[doc = "`reset()` method sets SBS to value 0"]
impl crate::Resettable for SBS_SPEC {
    const RESET_VALUE: u16 = 0;
}
