#[doc = "Register `DEBUG_IDCODE` reader"]
pub type R = crate::R<DEBUG_IDCODE_SPEC>;
#[doc = "Field `PID` reader - PID"]
pub type PID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(self.bits)
    }
}
#[doc = "DEBUG IDCODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_idcode::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_IDCODE_SPEC;
impl crate::RegisterSpec for DEBUG_IDCODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_idcode::R`](R) reader structure"]
impl crate::Readable for DEBUG_IDCODE_SPEC {}
#[doc = "`reset()` method sets DEBUG_IDCODE to value 0"]
impl crate::Resettable for DEBUG_IDCODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
