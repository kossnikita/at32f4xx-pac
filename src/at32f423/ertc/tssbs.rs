#[doc = "Register `TSSBS` reader"]
pub type R = crate::R<TSSBS_SPEC>;
#[doc = "Field `SBS` reader - Sub second value"]
pub type SBS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value"]
    #[inline(always)]
    pub fn sbs(&self) -> SBS_R {
        SBS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "timestamp sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tssbs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSSBS_SPEC;
impl crate::RegisterSpec for TSSBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tssbs::R`](R) reader structure"]
impl crate::Readable for TSSBS_SPEC {}
#[doc = "`reset()` method sets TSSBS to value 0"]
impl crate::Resettable for TSSBS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
