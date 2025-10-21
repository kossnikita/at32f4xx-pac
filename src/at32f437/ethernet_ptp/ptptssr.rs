#[doc = "Register `PTPTSSR` reader"]
pub type R = crate::R<PTPTSSR_SPEC>;
#[doc = "Field `TSO` reader - Timestamp second overflow"]
pub type TSO_R = crate::BitReader;
#[doc = "Field `TTTR` reader - Timestamp target time reached"]
pub type TTTR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timestamp second overflow"]
    #[inline(always)]
    pub fn tso(&self) -> TSO_R {
        TSO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp target time reached"]
    #[inline(always)]
    pub fn tttr(&self) -> TTTR_R {
        TTTR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSSR")
            .field("tso", &self.tso())
            .field("tttr", &self.tttr())
            .finish()
    }
}
#[doc = "Ethernet PTP time stamp status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSSR_SPEC;
impl crate::RegisterSpec for PTPTSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptssr::R`](R) reader structure"]
impl crate::Readable for PTPTSSR_SPEC {}
#[doc = "`reset()` method sets PTPTSSR to value 0"]
impl crate::Resettable for PTPTSSR_SPEC {}
