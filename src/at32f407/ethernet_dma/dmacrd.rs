#[doc = "Register `DMACRD` reader"]
pub type R = crate::R<DMACRD_SPEC>;
#[doc = "Field `HRDAP` reader - Host receive descriptor address pointer"]
pub type HRDAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host receive descriptor address pointer"]
    #[inline(always)]
    pub fn hrdap(&self) -> HRDAP_R {
        HRDAP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACRD")
            .field("hrdap", &self.hrdap())
            .finish()
    }
}
#[doc = "Ethernet DMA current host receive descriptor register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRD_SPEC;
impl crate::RegisterSpec for DMACRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrd::R`](R) reader structure"]
impl crate::Readable for DMACRD_SPEC {}
#[doc = "`reset()` method sets DMACRD to value 0"]
impl crate::Resettable for DMACRD_SPEC {}
