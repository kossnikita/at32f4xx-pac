#[doc = "Register `DMACRBADDR` reader"]
pub type R = crate::R<DMACRBADDR_SPEC>;
#[doc = "Field `HRBAP` reader - Host receive buffer address pointer"]
pub type HRBAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host receive buffer address pointer"]
    #[inline(always)]
    pub fn hrbap(&self) -> HRBAP_R {
        HRBAP_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host receive buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrbaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRBADDR_SPEC;
impl crate::RegisterSpec for DMACRBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrbaddr::R`](R) reader structure"]
impl crate::Readable for DMACRBADDR_SPEC {}
#[doc = "`reset()` method sets DMACRBADDR to value 0"]
impl crate::Resettable for DMACRBADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
