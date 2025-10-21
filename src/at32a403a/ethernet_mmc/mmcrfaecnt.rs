#[doc = "Register `MMCRFAECNT` reader"]
pub type R = crate::R<MMCRFAECNT_SPEC>;
#[doc = "Field `RFAEC` reader - Received frames alignment error counter"]
pub type RFAEC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received frames alignment error counter"]
    #[inline(always)]
    pub fn rfaec(&self) -> RFAEC_R {
        RFAEC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCRFAECNT")
            .field("rfaec", &self.rfaec())
            .finish()
    }
}
#[doc = "Ethernet MMC received frames with alignment error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmcrfaecnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRFAECNT_SPEC;
impl crate::RegisterSpec for MMCRFAECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrfaecnt::R`](R) reader structure"]
impl crate::Readable for MMCRFAECNT_SPEC {}
#[doc = "`reset()` method sets MMCRFAECNT to value 0"]
impl crate::Resettable for MMCRFAECNT_SPEC {}
