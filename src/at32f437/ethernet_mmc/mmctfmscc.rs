#[doc = "Register `MMCTFMSCC` reader"]
pub type R = crate::R<MMCTFMSCC_SPEC>;
#[doc = "Field `TGFMSCC` reader - Transmitted good frame more single collision counter"]
pub type TGFMSCC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frame more single collision counter"]
    #[inline(always)]
    pub fn tgfmscc(&self) -> TGFMSCC_R {
        TGFMSCC_R::new(self.bits)
    }
}
#[doc = "Ethernet MMC transmitted good frames after more than a single collision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctfmscc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTFMSCC_SPEC;
impl crate::RegisterSpec for MMCTFMSCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctfmscc::R`](R) reader structure"]
impl crate::Readable for MMCTFMSCC_SPEC {}
#[doc = "`reset()` method sets MMCTFMSCC to value 0"]
impl crate::Resettable for MMCTFMSCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
