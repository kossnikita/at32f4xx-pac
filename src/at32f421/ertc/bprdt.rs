#[doc = "Register `BPR%sDT` reader"]
pub type R = crate::R<BPRDT_SPEC>;
#[doc = "Register `BPR%sDT` writer"]
pub type W = crate::W<BPRDT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<BPRDT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bprdt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bprdt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BPRDT_SPEC;
impl crate::RegisterSpec for BPRDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bprdt::R`](R) reader structure"]
impl crate::Readable for BPRDT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bprdt::W`](W) writer structure"]
impl crate::Writable for BPRDT_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BPR%sDT to value 0"]
impl crate::Resettable for BPRDT_SPEC {
    const RESET_VALUE: u32 = 0;
}
