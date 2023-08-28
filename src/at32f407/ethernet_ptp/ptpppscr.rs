#[doc = "Register `PTPPPSCR` reader"]
pub type R = crate::R<PTPPPSCR_SPEC>;
#[doc = "Field `POFC` reader - PPS Output frequency control"]
pub type POFC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - PPS Output frequency control"]
    #[inline(always)]
    pub fn pofc(&self) -> POFC_R {
        POFC_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Ethernet PTP PPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpppscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPPPSCR_SPEC;
impl crate::RegisterSpec for PTPPPSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptpppscr::R`](R) reader structure"]
impl crate::Readable for PTPPPSCR_SPEC {}
#[doc = "`reset()` method sets PTPPPSCR to value 0"]
impl crate::Resettable for PTPPPSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
