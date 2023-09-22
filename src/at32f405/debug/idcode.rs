#[doc = "Register `IDCODE` reader"]
pub type R = crate::R<IDCODE_SPEC>;
#[doc = "Field `PID` reader - PID information"]
pub type PID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PID information"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(self.bits)
    }
}
#[doc = "DEBUG IDCODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idcode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDCODE_SPEC;
impl crate::RegisterSpec for IDCODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idcode::R`](R) reader structure"]
impl crate::Readable for IDCODE_SPEC {}
#[doc = "`reset()` method sets IDCODE to value 0"]
impl crate::Resettable for IDCODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
