#[doc = "Register `BUFCNTR` reader"]
pub type R = crate::R<BUFCNTR_SPEC>;
#[doc = "Field `CNT` reader - FIF0COUNT"]
pub type CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - FIF0COUNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUFCNTR").field("cnt", &self.cnt()).finish()
    }
}
#[doc = "Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`bufcntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUFCNTR_SPEC;
impl crate::RegisterSpec for BUFCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bufcntr::R`](R) reader structure"]
impl crate::Readable for BUFCNTR_SPEC {}
#[doc = "`reset()` method sets BUFCNTR to value 0"]
impl crate::Resettable for BUFCNTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
