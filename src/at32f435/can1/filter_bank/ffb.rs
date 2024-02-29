#[doc = "Register `FFB%s` reader"]
pub type R = crate::R<FFB_SPEC>;
#[doc = "Register `FFB%s` writer"]
pub type W = crate::W<FFB_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<FFB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "CAN filter bank filter bit register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ffb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ffb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FFB_SPEC;
impl crate::RegisterSpec for FFB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffb::R`](R) reader structure"]
impl crate::Readable for FFB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ffb::W`](W) writer structure"]
impl crate::Writable for FFB_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFB%s to value 0"]
impl crate::Resettable for FFB_SPEC {
    const RESET_VALUE: u32 = 0;
}
