#[doc = "Register `GNPTXSTS` reader"]
pub type R = crate::R<GNPTXSTS_SPEC>;
#[doc = "Field `NPTXFSPCAVAIL` reader - Non-periodic TxFIFO space available"]
pub type NPTXFSPCAVAIL_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXQSPCAVAIL` reader - Non-periodic transmit request queue space available"]
pub type NPTXQSPCAVAIL_R = crate::FieldReader;
#[doc = "Field `NPTXQTOP` reader - Top of the non-periodic transmit request queue"]
pub type NPTXQTOP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Non-periodic TxFIFO space available"]
    #[inline(always)]
    pub fn nptxfspcavail(&self) -> NPTXFSPCAVAIL_R {
        NPTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Non-periodic transmit request queue space available"]
    #[inline(always)]
    pub fn nptxqspcavail(&self) -> NPTXQSPCAVAIL_R {
        NPTXQSPCAVAIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the non-periodic transmit request queue"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GNPTXSTS")
            .field("nptxfspcavail", &self.nptxfspcavail())
            .field("nptxqspcavail", &self.nptxqspcavail())
            .field("nptxqtop", &self.nptxqtop())
            .finish()
    }
}
#[doc = "OTGHS non-periodic transmit FIFO/queue status register (OTGHS_GNPTXSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GNPTXSTS_SPEC;
impl crate::RegisterSpec for GNPTXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxsts::R`](R) reader structure"]
impl crate::Readable for GNPTXSTS_SPEC {}
#[doc = "`reset()` method sets GNPTXSTS to value 0x0008_0200"]
impl crate::Resettable for GNPTXSTS_SPEC {
    const RESET_VALUE: u32 = 0x0008_0200;
}
