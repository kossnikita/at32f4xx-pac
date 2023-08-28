#[doc = "Register `DTCNTR` reader"]
pub type R = crate::R<DTCNTR_SPEC>;
#[doc = "Field `CNT` reader - Data count value"]
pub type CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - Data count value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "Bits 24:0 = DATACOUNT: Data count value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtcntr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTCNTR_SPEC;
impl crate::RegisterSpec for DTCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtcntr::R`](R) reader structure"]
impl crate::Readable for DTCNTR_SPEC {}
#[doc = "`reset()` method sets DTCNTR to value 0"]
impl crate::Resettable for DTCNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
