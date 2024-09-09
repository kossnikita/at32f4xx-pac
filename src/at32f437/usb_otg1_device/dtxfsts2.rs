#[doc = "Register `DTXFSTS2` reader"]
pub type R = crate::R<DTXFSTS2_SPEC>;
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
        f.debug_struct("DTXFSTS2")
            .field("ineptxfsav", &self.ineptxfsav())
            .finish()
    }
}
#[doc = "OTGFS device IN endpoint-2 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS2_SPEC;
impl crate::RegisterSpec for DTXFSTS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts2::R`](R) reader structure"]
impl crate::Readable for DTXFSTS2_SPEC {}
#[doc = "`reset()` method sets DTXFSTS2 to value 0"]
impl crate::Resettable for DTXFSTS2_SPEC {
    const RESET_VALUE: u32 = 0;
}
