#[doc = "Register `FIFOSTS` reader"]
pub type R = crate::R<FIFOSTS_SPEC>;
#[doc = "Field `TXFIFORDY` reader - TxFIFO ready status"]
pub type TXFIFORDY_R = crate::BitReader;
#[doc = "Field `RXFIFORDY` reader - RxFIFO ready status"]
pub type RXFIFORDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TxFIFO ready status"]
    #[inline(always)]
    pub fn txfifordy(&self) -> TXFIFORDY_R {
        TXFIFORDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RxFIFO ready status"]
    #[inline(always)]
    pub fn rxfifordy(&self) -> RXFIFORDY_R {
        RXFIFORDY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFOSTS")
            .field("txfifordy", &format_args!("{}", self.txfifordy().bit()))
            .field("rxfifordy", &format_args!("{}", self.rxfifordy().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<FIFOSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "FIFO Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifosts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFOSTS_SPEC;
impl crate::RegisterSpec for FIFOSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifosts::R`](R) reader structure"]
impl crate::Readable for FIFOSTS_SPEC {}
#[doc = "`reset()` method sets FIFOSTS to value 0x01"]
impl crate::Resettable for FIFOSTS_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
