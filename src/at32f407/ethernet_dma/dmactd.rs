#[doc = "Register `DMACTD` reader"]
pub type R = crate::R<DMACTD_SPEC>;
#[doc = "Field `HTDAP` reader - Host transmit descriptor address pointer"]
pub type HTDAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host transmit descriptor address pointer"]
    #[inline(always)]
    pub fn htdap(&self) -> HTDAP_R {
        HTDAP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACTD")
            .field("htdap", &self.htdap())
            .finish()
    }
}
#[doc = "Ethernet DMA current host transmit descriptor register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTD_SPEC;
impl crate::RegisterSpec for DMACTD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactd::R`](R) reader structure"]
impl crate::Readable for DMACTD_SPEC {}
#[doc = "`reset()` method sets DMACTD to value 0"]
impl crate::Resettable for DMACTD_SPEC {}
