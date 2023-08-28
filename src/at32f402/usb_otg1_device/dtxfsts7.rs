#[doc = "Register `DTXFSTS7` reader"]
pub type R = crate::R<DTXFSTS7_SPEC>;
#[doc = "Field `INEPTXFSAV` reader - IN endpoint TxFIFO space available"]
pub type INEPTXFSAV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space available"]
    #[inline(always)]
    pub fn ineptxfsav(&self) -> INEPTXFSAV_R {
        INEPTXFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTGFS device IN endpoint-7 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS7_SPEC;
impl crate::RegisterSpec for DTXFSTS7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts7::R`](R) reader structure"]
impl crate::Readable for DTXFSTS7_SPEC {}
#[doc = "`reset()` method sets DTXFSTS7 to value 0"]
impl crate::Resettable for DTXFSTS7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
