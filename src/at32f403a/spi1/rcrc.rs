#[doc = "Register `RCRC` reader"]
pub type R = crate::R<RCRC_SPEC>;
#[doc = "Field `RCRC` reader - Receive CRC"]
pub type RCRC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Receive CRC"]
    #[inline(always)]
    pub fn rcrc(&self) -> RCRC_R {
        RCRC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcrc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCRC_SPEC;
impl crate::RegisterSpec for RCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcrc::R`](R) reader structure"]
impl crate::Readable for RCRC_SPEC {}
#[doc = "`reset()` method sets RCRC to value 0"]
impl crate::Resettable for RCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
