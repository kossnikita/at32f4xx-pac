#[doc = "Register `EPPS0` reader"]
pub type R = crate::R<EPPS0_SPEC>;
#[doc = "Field `EPPS` reader - Erase/program protection status"]
pub type EPPS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Erase/program protection status"]
    #[inline(always)]
    pub fn epps(&self) -> EPPS_R {
        EPPS_R::new(self.bits)
    }
}
#[doc = "Erase/program protection status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epps0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPPS0_SPEC;
impl crate::RegisterSpec for EPPS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epps0::R`](R) reader structure"]
impl crate::Readable for EPPS0_SPEC {}
#[doc = "`reset()` method sets EPPS0 to value 0xffff_ffff"]
impl crate::Resettable for EPPS0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
