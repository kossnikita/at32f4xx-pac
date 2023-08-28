#[doc = "Register `EPPS` reader"]
pub type R = crate::R<EPPS_SPEC>;
#[doc = "Field `EPPS` reader - Erase/program protection status"]
pub type EPPS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Erase/program protection status"]
    #[inline(always)]
    pub fn epps(&self) -> EPPS_R {
        EPPS_R::new(self.bits)
    }
}
#[doc = "Erase/program protection status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epps::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPPS_SPEC;
impl crate::RegisterSpec for EPPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epps::R`](R) reader structure"]
impl crate::Readable for EPPS_SPEC {}
#[doc = "`reset()` method sets EPPS to value 0xffff_ffff"]
impl crate::Resettable for EPPS_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
