#[doc = "Register `DTXFSTS3` reader"]
pub type R = crate::R<DTXFSTS3_SPEC>;
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
        f.debug_struct("DTXFSTS3")
            .field("ineptxfsav", &format_args!("{}", self.ineptxfsav().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DTXFSTS3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "OTGFS device IN endpoint-3 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS3_SPEC;
impl crate::RegisterSpec for DTXFSTS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts3::R`](R) reader structure"]
impl crate::Readable for DTXFSTS3_SPEC {}
#[doc = "`reset()` method sets DTXFSTS3 to value 0"]
impl crate::Resettable for DTXFSTS3_SPEC {
    const RESET_VALUE: u32 = 0;
}
