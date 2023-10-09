#[doc = "Register `EPPS1` reader"]
pub type R = crate::R<EPPS1_SPEC>;
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
        f.debug_struct("EPPS1")
            .field("epps", &format_args!("{}", self.epps().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<EPPS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Erase/program protection status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epps1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPPS1_SPEC;
impl crate::RegisterSpec for EPPS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epps1::R`](R) reader structure"]
impl crate::Readable for EPPS1_SPEC {}
#[doc = "`reset()` method sets EPPS1 to value 0xffff_ffff"]
impl crate::Resettable for EPPS1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
