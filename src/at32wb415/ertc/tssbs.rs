#[doc = "Register `TSSBS` reader"]
pub type R = crate::R<TSSBS_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TSSBS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
