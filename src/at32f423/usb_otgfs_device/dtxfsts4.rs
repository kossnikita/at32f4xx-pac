#[doc = "Register `DTXFSTS4` reader"]
pub type R = crate::R<DTXFSTS4_SPEC>;
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
        f.debug_struct("DTXFSTS4")
            .field("ineptxfsav", &format_args!("{}", self.ineptxfsav().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DTXFSTS4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "OTGFS device IN endpoint-4 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS4_SPEC;
impl crate::RegisterSpec for DTXFSTS4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts4::R`](R) reader structure"]
impl crate::Readable for DTXFSTS4_SPEC {}
#[doc = "`reset()` method sets DTXFSTS4 to value 0"]
impl crate::Resettable for DTXFSTS4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
