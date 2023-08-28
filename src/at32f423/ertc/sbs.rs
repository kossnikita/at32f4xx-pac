#[doc = "Register `SBS` reader"]
pub type R = crate::R<SBS_SPEC>;
#[doc = "Field `SBS` reader - Sub second value"]
pub type SBS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value"]
    #[inline(always)]
    pub fn sbs(&self) -> SBS_R {
        SBS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sbs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SBS_SPEC;
impl crate::RegisterSpec for SBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbs::R`](R) reader structure"]
impl crate::Readable for SBS_SPEC {}
#[doc = "`reset()` method sets SBS to value 0"]
impl crate::Resettable for SBS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
