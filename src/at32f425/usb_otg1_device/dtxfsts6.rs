#[doc = "Register `DTXFSTS6` reader"]
pub type R = crate::R<DTXFSTS6_SPEC>;
#[doc = "Field `INEPTXFSAV` reader - IN endpoint TxFIFO space available"]
pub type INEPTXFSAV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space available"]
    #[inline(always)]
    pub fn ineptxfsav(&self) -> INEPTXFSAV_R {
        INEPTXFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTXFSTS6")
            .field("ineptxfsav", &self.ineptxfsav())
            .finish()
    }
}
#[doc = "OTGFS device IN endpoint-6 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS6_SPEC;
impl crate::RegisterSpec for DTXFSTS6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts6::R`](R) reader structure"]
impl crate::Readable for DTXFSTS6_SPEC {}
#[doc = "`reset()` method sets DTXFSTS6 to value 0"]
impl crate::Resettable for DTXFSTS6_SPEC {}
