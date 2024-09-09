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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPPS0").field("epps", &self.epps()).finish()
    }
}
#[doc = "Erase/program protection status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`epps0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPPS0_SPEC;
impl crate::RegisterSpec for EPPS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epps0::R`](R) reader structure"]
impl crate::Readable for EPPS0_SPEC {}
#[doc = "`reset()` method sets EPPS0 to value 0xffff_ffff"]
impl crate::Resettable for EPPS0_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
