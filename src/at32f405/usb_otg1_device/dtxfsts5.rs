#[doc = "Register `DTXFSTS5` reader"]
pub type R = crate::R<DTXFSTS5_SPEC>;
#[doc = "Field `INEPTXFSAV` reader - IN endpoint TxFIFO space available"]
pub type INEPTXFSAV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space available"]
    #[inline(always)]
    pub fn ineptxfsav(&self) -> INEPTXFSAV_R {
        INEPTXFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTGFS device IN endpoint-5 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS5_SPEC;
impl crate::RegisterSpec for DTXFSTS5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts5::R`](R) reader structure"]
impl crate::Readable for DTXFSTS5_SPEC {}
#[doc = "`reset()` method sets DTXFSTS5 to value 0"]
impl crate::Resettable for DTXFSTS5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
