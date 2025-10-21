#[doc = "Register `DMACTBADDR` reader"]
pub type R = crate::R<DMACTBADDR_SPEC>;
#[doc = "Field `HTBAP` reader - Host transmit buffer address pointer"]
pub type HTBAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host transmit buffer address pointer"]
    #[inline(always)]
    pub fn htbap(&self) -> HTBAP_R {
        HTBAP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACTBADDR")
            .field("htbap", &self.htbap())
            .finish()
    }
}
#[doc = "Ethernet DMA current host transmit buffer address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactbaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTBADDR_SPEC;
impl crate::RegisterSpec for DMACTBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactbaddr::R`](R) reader structure"]
impl crate::Readable for DMACTBADDR_SPEC {}
#[doc = "`reset()` method sets DMACTBADDR to value 0"]
impl crate::Resettable for DMACTBADDR_SPEC {}
